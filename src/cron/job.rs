//! Job definitions and management for cron scheduler
//!
//! This module provides the Job struct and related functionality
//! for defining and executing scheduled tasks.

use crate::error::{Error, Result};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(feature = "tokio")]
use tokio::sync::RwLock;

/// Type alias for sync job functions
pub type SyncJobFn = dyn Fn() -> Result<()> + Send + Sync + 'static;

/// Type alias for async job functions
pub type AsyncJobFn =
    dyn Fn() -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>> + Send + Sync + 'static;

/// A job that can be scheduled and executed by the scheduler
#[derive(Clone)]
pub struct Job {
    /// Human-readable name for the job
    pub name: String,
    /// Optional description of what the job does
    pub description: Option<String>,
    /// The function to execute
    job_fn: JobFunction,
    /// Job metadata
    metadata: JobMetadata,
}

/// Job execution function variants
#[derive(Clone)]
enum JobFunction {
    /// Synchronous job function
    Sync(Arc<SyncJobFn>),
    /// Asynchronous job function
    Async(Arc<AsyncJobFn>),
}

/// Metadata associated with a job
#[derive(Debug, Clone)]
pub struct JobMetadata {
    /// Job category/group
    pub category: Option<String>,
    /// Job priority (higher numbers = higher priority)
    pub priority: i32,
    /// Maximum execution time allowed
    pub timeout: Option<Duration>,
    /// Number of retry attempts on failure
    pub max_retries: u32,
    /// Whether to retry on failure
    pub retry_on_failure: bool,
    /// Custom tags for job organization
    pub tags: Vec<String>,
}

impl Default for JobMetadata {
    fn default() -> Self {
        Self {
            category: None,
            priority: 0,
            timeout: Some(Duration::from_secs(300)), // 5 minutes default
            max_retries: 0,
            retry_on_failure: false,
            tags: Vec::new(),
        }
    }
}

/// Job execution result with timing information
#[derive(Debug)]
pub struct JobResult {
    /// Whether the job succeeded
    pub success: bool,
    /// Error message if the job failed
    pub error: Option<String>,
    /// Time taken to execute the job
    pub duration: Duration,
    /// Timestamp when the job started
    pub started_at: Instant,
    /// Timestamp when the job completed
    pub completed_at: Instant,
}

impl Job {
    /// Create a new synchronous job
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::Job;
    ///
    /// let job = Job::new("daily_backup", Box::new(|| {
    ///     println!("Running daily backup...");
    ///     // Backup logic here
    ///     Ok(())
    /// }));
    /// ```
    pub fn new<F>(name: &str, job_fn: Box<F>) -> Self
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            description: None,
            job_fn: JobFunction::Sync(Arc::new(*job_fn)),
            metadata: JobMetadata::default(),
        }
    }

    /// Create a new asynchronous job
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::Job;
    /// use std::future::Future;
    /// use std::pin::Pin;
    ///
    /// let job = Job::new_async("api_sync", Box::new(|| {
    ///     Box::pin(async {
    ///         println!("Syncing with API...");
    ///         // Async API call here
    ///         Ok(())
    ///     })
    /// }));
    /// ```
    pub fn new_async<F, Fut>(name: &str, job_fn: Box<F>) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let async_fn = Arc::new(
            move || -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>> {
                Box::pin(job_fn())
            },
        );

        Self {
            name: name.to_string(),
            description: None,
            job_fn: JobFunction::Async(async_fn),
            metadata: JobMetadata::default(),
        }
    }

    /// Set the job description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set the job category
    pub fn with_category(mut self, category: &str) -> Self {
        self.metadata.category = Some(category.to_string());
        self
    }

    /// Set the job priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.metadata.priority = priority;
        self
    }

    /// Set the job timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.metadata.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.metadata.max_retries = max_retries;
        self.metadata.retry_on_failure = max_retries > 0;
        self
    }

    /// Add tags to the job
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.metadata.tags = tags;
        self
    }

    /// Add a single tag to the job
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.metadata.tags.push(tag.to_string());
        self
    }

    /// Execute the job
    #[cfg(feature = "tokio")]
    pub async fn execute(&self) -> Result<()> {
        let start_time = Instant::now();

        let result = match &self.job_fn {
            JobFunction::Sync(job_fn) => {
                // Execute sync job in a blocking task
                let job_fn = job_fn.clone();
                tokio::task::spawn_blocking(move || job_fn())
                    .await
                    .map_err(|e| Error::custom(format!("Job execution failed: {}", e)))?
            }
            JobFunction::Async(job_fn) => {
                // Execute async job directly
                job_fn().await
            }
        };

        // Apply timeout if specified
        if let Some(timeout) = self.metadata.timeout {
            if start_time.elapsed() > timeout {
                return Err(Error::timeout(format!(
                    "Job '{}' exceeded timeout of {:?}",
                    self.name, timeout
                )));
            }
        }

        result
    }

    /// Execute the job with retries
    #[cfg(feature = "tokio")]
    pub async fn execute_with_retries(&self) -> JobResult {
        let started_at = Instant::now();
        let mut last_error = None;

        for attempt in 0..=self.metadata.max_retries {
            let _attempt_start = Instant::now();

            match self.execute().await {
                Ok(_) => {
                    return JobResult {
                        success: true,
                        error: None,
                        duration: started_at.elapsed(),
                        started_at,
                        completed_at: Instant::now(),
                    };
                }
                Err(e) => {
                    last_error = Some(e.to_string());

                    // Don't sleep after the last attempt
                    if attempt < self.metadata.max_retries {
                        // Exponential backoff: 1s, 2s, 4s, 8s, ...
                        let delay = Duration::from_secs(2_u64.pow(attempt));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        JobResult {
            success: false,
            error: last_error,
            duration: started_at.elapsed(),
            started_at,
            completed_at: Instant::now(),
        }
    }

    /// Get job metadata
    pub fn metadata(&self) -> &JobMetadata {
        &self.metadata
    }

    /// Check if this job has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.metadata.tags.contains(&tag.to_string())
    }

    /// Get job name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get job description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Job")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Job: {}", self.name)?;
        if let Some(ref description) = self.description {
            write!(f, " ({})", description)?;
        }
        if let Some(ref category) = self.metadata.category {
            write!(f, " [{}]", category)?;
        }
        Ok(())
    }
}

/// Builder for creating jobs with fluent interface
pub struct JobBuilder {
    name: String,
    description: Option<String>,
    metadata: JobMetadata,
}

impl JobBuilder {
    /// Create a new job builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::JobBuilder;
    /// use std::time::Duration;
    ///
    /// let job = JobBuilder::new("cleanup_job")
    ///     .description("Clean up temporary files")
    ///     .category("maintenance")
    ///     .priority(5)
    ///     .timeout(Duration::from_secs(120))
    ///     .max_retries(3)
    ///     .tag("cleanup")
    ///     .tag("maintenance")
    ///     .build_sync(Box::new(|| {
    ///         println!("Cleaning up...");
    ///         Ok(())
    ///     }));
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            metadata: JobMetadata::default(),
        }
    }

    /// Set the job description
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set the job category
    pub fn category(mut self, category: &str) -> Self {
        self.metadata.category = Some(category.to_string());
        self
    }

    /// Set the job priority
    pub fn priority(mut self, priority: i32) -> Self {
        self.metadata.priority = priority;
        self
    }

    /// Set the job timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.metadata.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.metadata.max_retries = max_retries;
        self.metadata.retry_on_failure = max_retries > 0;
        self
    }

    /// Add a tag to the job
    pub fn tag(mut self, tag: &str) -> Self {
        self.metadata.tags.push(tag.to_string());
        self
    }

    /// Add multiple tags to the job
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.metadata.tags.extend(tags);
        self
    }

    /// Build a synchronous job
    pub fn build_sync<F>(self, job_fn: Box<F>) -> Job
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        Job {
            name: self.name,
            description: self.description,
            job_fn: JobFunction::Sync(Arc::new(*job_fn)),
            metadata: self.metadata,
        }
    }

    /// Build an asynchronous job
    pub fn build_async<F, Fut>(self, job_fn: Box<F>) -> Job
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let async_fn = Arc::new(
            move || -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>> {
                Box::pin(job_fn())
            },
        );

        Job {
            name: self.name,
            description: self.description,
            job_fn: JobFunction::Async(async_fn),
            metadata: self.metadata,
        }
    }
}

/// Job registry for managing and organizing jobs
#[derive(Debug)]
pub struct JobRegistry {
    jobs: Arc<RwLock<Vec<Job>>>,
}

impl JobRegistry {
    /// Create a new job registry
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a job in the registry
    #[cfg(feature = "tokio")]
    pub async fn register(&self, job: Job) -> Result<()> {
        let mut jobs = self.jobs.write().await;

        // Check if job with same name already exists
        if jobs.iter().any(|j| j.name == job.name) {
            return Err(Error::validation(format!(
                "Job with name '{}' already exists",
                job.name
            )));
        }

        jobs.push(job);
        Ok(())
    }

    /// Get a job by name
    #[cfg(feature = "tokio")]
    pub async fn get(&self, name: &str) -> Option<Job> {
        let jobs = self.jobs.read().await;
        jobs.iter().find(|j| j.name == name).cloned()
    }

    /// Get all jobs in a category
    #[cfg(feature = "tokio")]
    pub async fn get_by_category(&self, category: &str) -> Vec<Job> {
        let jobs = self.jobs.read().await;
        jobs.iter()
            .filter(|j| j.metadata.category.as_deref() == Some(category))
            .cloned()
            .collect()
    }

    /// Get all jobs with a specific tag
    #[cfg(feature = "tokio")]
    pub async fn get_by_tag(&self, tag: &str) -> Vec<Job> {
        let jobs = self.jobs.read().await;
        jobs.iter().filter(|j| j.has_tag(tag)).cloned().collect()
    }

    /// Get all jobs ordered by priority (highest first)
    #[cfg(feature = "tokio")]
    pub async fn get_by_priority(&self) -> Vec<Job> {
        let jobs = self.jobs.read().await;
        let mut sorted_jobs = jobs.clone();
        sorted_jobs.sort_by(|a, b| b.metadata.priority.cmp(&a.metadata.priority));
        sorted_jobs
    }

    /// Remove a job by name
    #[cfg(feature = "tokio")]
    pub async fn remove(&self, name: &str) -> Result<Job> {
        let mut jobs = self.jobs.write().await;

        if let Some(pos) = jobs.iter().position(|j| j.name == name) {
            Ok(jobs.remove(pos))
        } else {
            Err(Error::not_found(format!("Job '{}' not found", name)))
        }
    }

    /// List all job names
    #[cfg(feature = "tokio")]
    pub async fn list_names(&self) -> Vec<String> {
        let jobs = self.jobs.read().await;
        jobs.iter().map(|j| j.name.clone()).collect()
    }

    /// Get the number of registered jobs
    #[cfg(feature = "tokio")]
    pub async fn count(&self) -> usize {
        let jobs = self.jobs.read().await;
        jobs.len()
    }

    /// Clear all jobs from the registry
    #[cfg(feature = "tokio")]
    pub async fn clear(&self) -> Result<()> {
        let mut jobs = self.jobs.write().await;
        jobs.clear();
        Ok(())
    }
}

impl Default for JobRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JobResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.success {
            write!(f, "✓ Success ({}ms)", self.duration.as_millis())
        } else {
            write!(
                f,
                "✗ Failed: {} ({}ms)",
                self.error.as_ref().unwrap_or(&"Unknown error".to_string()),
                self.duration.as_millis()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_job_creation() {
        let job = Job::new("test_job", Box::new(|| Ok(())));
        assert_eq!(job.name(), "test_job");
        assert!(job.description().is_none());
        assert_eq!(job.metadata().priority, 0);
    }

    #[test]
    fn test_job_with_metadata() {
        let job = Job::new("test_job", Box::new(|| Ok(())))
            .with_description("A test job")
            .with_category("testing")
            .with_priority(5)
            .with_timeout(Duration::from_secs(60))
            .with_tag("test")
            .with_tag("unit");

        assert_eq!(job.description(), Some("A test job"));
        assert_eq!(job.metadata().category, Some("testing".to_string()));
        assert_eq!(job.metadata().priority, 5);
        assert_eq!(job.metadata().timeout, Some(Duration::from_secs(60)));
        assert!(job.has_tag("test"));
        assert!(job.has_tag("unit"));
        assert!(!job.has_tag("integration"));
    }

    #[test]
    fn test_job_builder() {
        let job = JobBuilder::new("builder_job")
            .description("Built with builder")
            .category("builder")
            .priority(10)
            .tag("builder")
            .tag("test")
            .build_sync(Box::new(|| Ok(())));

        assert_eq!(job.name(), "builder_job");
        assert_eq!(job.description(), Some("Built with builder"));
        assert_eq!(job.metadata().category, Some("builder".to_string()));
        assert_eq!(job.metadata().priority, 10);
        assert!(job.has_tag("builder"));
        assert!(job.has_tag("test"));
    }

    #[tokio::test]
    async fn test_job_execution() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let job = Job::new(
            "counter_job",
            Box::new(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }),
        );

        assert_eq!(counter.load(Ordering::SeqCst), 0);
        job.execute().await.unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_job_async_execution() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let job = Job::new_async(
            "async_counter_job",
            Box::new(move || {
                let counter = counter_clone.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            }),
        );

        assert_eq!(counter.load(Ordering::SeqCst), 0);
        job.execute().await.unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_job_retries() {
        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let job = Job::new(
            "failing_job",
            Box::new(move || {
                let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err(Error::custom("Simulated failure".to_string()))
                } else {
                    Ok(())
                }
            }),
        )
        .with_max_retries(3);

        let result = job.execute_with_retries().await;
        assert!(result.success);
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3); // Initial + 2 retries
    }

    #[tokio::test]
    async fn test_job_registry() {
        let registry = JobRegistry::new();

        let job1 = Job::new("job1", Box::new(|| Ok(()))).with_category("test");
        let job2 = Job::new("job2", Box::new(|| Ok(()))).with_category("prod");
        let job3 = Job::new("job3", Box::new(|| Ok(()))).with_tag("important");

        registry.register(job1).await.unwrap();
        registry.register(job2).await.unwrap();
        registry.register(job3).await.unwrap();

        assert_eq!(registry.count().await, 3);

        let test_jobs = registry.get_by_category("test").await;
        assert_eq!(test_jobs.len(), 1);
        assert_eq!(test_jobs[0].name(), "job1");

        let important_jobs = registry.get_by_tag("important").await;
        assert_eq!(important_jobs.len(), 1);
        assert_eq!(important_jobs[0].name(), "job3");

        let job = registry.get("job2").await;
        assert!(job.is_some());
        assert_eq!(job.unwrap().name(), "job2");

        registry.remove("job1").await.unwrap();
        assert_eq!(registry.count().await, 2);
    }

    #[tokio::test]
    async fn test_job_registry_duplicate_name() {
        let registry = JobRegistry::new();

        let job1 = Job::new("duplicate", Box::new(|| Ok(())));
        let job2 = Job::new("duplicate", Box::new(|| Ok(())));

        registry.register(job1).await.unwrap();
        let result = registry.register(job2).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_job_display() {
        let job = Job::new("display_job", Box::new(|| Ok(())))
            .with_description("Test display")
            .with_category("display");

        let display = job.to_string();
        assert!(display.contains("display_job"));
        assert!(display.contains("Test display"));
        assert!(display.contains("[display]"));
    }

    #[test]
    fn test_job_result_display() {
        let success_result = JobResult {
            success: true,
            error: None,
            duration: Duration::from_millis(150),
            started_at: Instant::now(),
            completed_at: Instant::now(),
        };

        let fail_result = JobResult {
            success: false,
            error: Some("Test error".to_string()),
            duration: Duration::from_millis(75),
            started_at: Instant::now(),
            completed_at: Instant::now(),
        };

        let success_display = success_result.to_string();
        let fail_display = fail_result.to_string();

        assert!(success_display.contains("✓ Success"));
        assert!(success_display.contains("150ms"));

        assert!(fail_display.contains("✗ Failed"));
        assert!(fail_display.contains("Test error"));
        assert!(fail_display.contains("75ms"));
    }
}
