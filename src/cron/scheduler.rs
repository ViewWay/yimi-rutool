//! Task scheduler for cron jobs
//!
//! This module provides a comprehensive task scheduler that can execute
//! jobs based on cron expressions with support for async operations.

use crate::error::{Error, Result};
use crate::cron::cron_parser::CronExpression;
use crate::cron::job::Job;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[cfg(feature = "tokio")]
use tokio::sync::mpsc;
#[cfg(feature = "tokio")]
use tokio::task::JoinHandle;
#[cfg(feature = "tokio")]
use tokio::time::interval;

/// A task scheduler that manages and executes cron jobs
#[derive(Debug)]
pub struct Scheduler {
    /// Storage for scheduled jobs
    jobs: Arc<Mutex<HashMap<String, ScheduledJob>>>,
    /// Whether the scheduler is running
    is_running: Arc<AtomicBool>,
    /// Next job ID to assign
    next_job_id: Arc<AtomicU64>,
    /// Scheduler configuration
    config: SchedulerConfig,
    /// Task handle for the main scheduler loop
    #[cfg(feature = "tokio")]
    task_handle: Option<JoinHandle<()>>,
    /// Shutdown signal sender
    #[cfg(feature = "tokio")]
    shutdown_tx: Option<mpsc::UnboundedSender<()>>,
}

/// Configuration for the scheduler
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// How often to check for jobs to run (in milliseconds)
    pub tick_interval: Duration,
    /// Maximum number of concurrent jobs
    pub max_concurrent_jobs: usize,
    /// Whether to run missed jobs immediately
    pub run_missed_jobs: bool,
    /// Timezone for scheduling (defaults to UTC)
    pub timezone: String,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_millis(1000), // Check every second
            max_concurrent_jobs: 100,
            run_missed_jobs: true,
            timezone: "UTC".to_string(),
        }
    }
}

/// A job scheduled in the scheduler
#[derive(Debug)]
struct ScheduledJob {
    /// Unique identifier for the job
    id: String,
    /// The job to execute
    job: Job,
    /// Cron expression for scheduling
    cron_expr: CronExpression,
    /// Next execution time
    #[cfg(feature = "chrono")]
    next_run: Option<DateTime<Utc>>,
    /// Last execution time
    #[cfg(feature = "chrono")]
    last_run: Option<DateTime<Utc>>,
    /// Whether this job is enabled
    enabled: bool,
    /// Number of times this job has been executed
    execution_count: u64,
    /// Whether this job is currently running
    is_running: bool,
}

/// Handle to a scheduled task that can be used to control it
#[derive(Debug, Clone)]
pub struct TaskHandle {
    /// Job ID
    pub id: String,
    /// Reference to the scheduler
    scheduler: Arc<Mutex<HashMap<String, ScheduledJob>>>,
}

impl Scheduler {
    /// Create a new scheduler with default configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::Scheduler;
    ///
    /// let scheduler = Scheduler::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(SchedulerConfig::default())
    }

    /// Create a new scheduler with custom configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::{Scheduler, SchedulerConfig};
    /// use std::time::Duration;
    ///
    /// let config = SchedulerConfig {
    ///     tick_interval: Duration::from_millis(500),
    ///     max_concurrent_jobs: 50,
    ///     ..Default::default()
    /// };
    /// let scheduler = Scheduler::with_config(config);
    /// ```
    pub fn with_config(config: SchedulerConfig) -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            next_job_id: Arc::new(AtomicU64::new(1)),
            config,
            #[cfg(feature = "tokio")]
            task_handle: None,
            #[cfg(feature = "tokio")]
            shutdown_tx: None,
        }
    }

    /// Add a job to the scheduler
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::{Scheduler, Job, CronExpression};
    ///
    /// let mut scheduler = Scheduler::new();
    /// let job = Job::new("test_job", Box::new(|| {
    ///     println!("Job executed!");
    ///     Ok(())
    /// }));
    /// let cron_expr = CronExpression::parse("0 * * * *").unwrap(); // Every hour
    /// 
    /// let handle = scheduler.add_job("my_job", job, cron_expr).unwrap();
    /// ```
    pub fn add_job(&mut self, name: &str, job: Job, cron_expr: CronExpression) -> Result<TaskHandle> {
        let job_id = format!("{}_{}", name, self.next_job_id.fetch_add(1, Ordering::SeqCst));
        
        // Validate the cron expression
        cron_expr.validate()?;

        #[cfg(feature = "chrono")]
        let next_run = cron_expr.next_execution(&Utc::now());

        let scheduled_job = ScheduledJob {
            id: job_id.clone(),
            job,
            cron_expr,
            #[cfg(feature = "chrono")]
            next_run,
            #[cfg(feature = "chrono")]
            last_run: None,
            enabled: true,
            execution_count: 0,
            is_running: false,
        };

        if let Ok(mut jobs) = self.jobs.lock() {
            jobs.insert(job_id.clone(), scheduled_job);
        } else {
            return Err(Error::concurrency("Failed to acquire jobs lock".to_string()));
        }

        Ok(TaskHandle {
            id: job_id,
            scheduler: self.jobs.clone(),
        })
    }

    /// Remove a job from the scheduler
    pub fn remove_job(&mut self, job_id: &str) -> Result<()> {
        if let Ok(mut jobs) = self.jobs.lock() {
            jobs.remove(job_id);
            Ok(())
        } else {
            Err(Error::concurrency("Failed to acquire jobs lock".to_string()))
        }
    }

    /// Start the scheduler
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::Scheduler;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut scheduler = Scheduler::new();
    ///     scheduler.start().await?;
    ///     
    ///     // Scheduler is now running in background
    ///     
    ///     scheduler.stop().await?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "tokio")]
    pub async fn start(&mut self) -> Result<()> {
        if self.is_running.load(Ordering::SeqCst) {
            return Err(Error::validation("Scheduler is already running".to_string()));
        }

        self.is_running.store(true, Ordering::SeqCst);

        let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
        self.shutdown_tx = Some(shutdown_tx);

        let jobs = self.jobs.clone();
        let is_running = self.is_running.clone();
        let tick_interval = self.config.tick_interval;
        let run_missed_jobs = self.config.run_missed_jobs;

        let task_handle = tokio::spawn(async move {
            let mut interval = interval(tick_interval);
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if !is_running.load(Ordering::SeqCst) {
                            break;
                        }

                        // Check for jobs to execute
                        Self::check_and_execute_jobs(&jobs, run_missed_jobs).await;
                    }
                    _ = shutdown_rx.recv() => {
                        break;
                    }
                }
            }
        });

        self.task_handle = Some(task_handle);
        Ok(())
    }

    /// Stop the scheduler
    #[cfg(feature = "tokio")]
    pub async fn stop(&mut self) -> Result<()> {
        if !self.is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.is_running.store(false, Ordering::SeqCst);

        // Send shutdown signal
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }

        // Wait for the task to complete
        if let Some(task_handle) = self.task_handle.take() {
            let _ = task_handle.await;
        }

        Ok(())
    }

    /// Check for jobs that need to be executed and run them
    #[cfg(feature = "tokio")]
    async fn check_and_execute_jobs(
        jobs: &Arc<Mutex<HashMap<String, ScheduledJob>>>,
        run_missed_jobs: bool,
    ) {
        let now = Utc::now();
        let mut jobs_to_execute = Vec::new();

        // Collect jobs that need to be executed
        if let Ok(mut jobs_guard) = jobs.lock() {
            for (job_id, scheduled_job) in jobs_guard.iter_mut() {
                if !scheduled_job.enabled || scheduled_job.is_running {
                    continue;
                }

                #[cfg(feature = "chrono")]
                if let Some(next_run) = scheduled_job.next_run {
                    let should_run = if run_missed_jobs {
                        next_run <= now
                    } else {
                        next_run <= now && (now - next_run).num_seconds() < 60 // Within 1 minute
                    };

                    if should_run {
                        scheduled_job.is_running = true;
                        scheduled_job.last_run = Some(now);
                        scheduled_job.execution_count += 1;
                        scheduled_job.next_run = scheduled_job.cron_expr.next_execution(&now);
                        
                        jobs_to_execute.push((job_id.clone(), scheduled_job.job.clone()));
                    }
                }
            }
        }

        // Execute jobs concurrently
        let mut handles = Vec::new();
        for (job_id, job) in jobs_to_execute {
            let jobs_ref = jobs.clone();
            let handle = tokio::spawn(async move {
                let start_time = Instant::now();
                let result = job.execute().await;
                let duration = start_time.elapsed();

                // Mark job as not running
                if let Ok(mut jobs_guard) = jobs_ref.lock() {
                    if let Some(scheduled_job) = jobs_guard.get_mut(&job_id) {
                        scheduled_job.is_running = false;
                    }
                }

                // Log execution result
                match result {
                    Ok(_) => {
                        println!("Job {} completed successfully in {:?}", job_id, duration);
                    }
                    Err(e) => {
                        eprintln!("Job {} failed: {} (duration: {:?})", job_id, e, duration);
                    }
                }
            });
            handles.push(handle);
        }

        // Wait for all jobs to complete
        for handle in handles {
            let _ = handle.await;
        }
    }

    /// Get information about all scheduled jobs
    pub fn get_jobs_info(&self) -> Result<Vec<JobInfo>> {
        if let Ok(jobs) = self.jobs.lock() {
            let mut job_infos = Vec::new();
            for scheduled_job in jobs.values() {
                job_infos.push(JobInfo {
                    id: scheduled_job.id.clone(),
                    name: scheduled_job.job.name.clone(),
                    cron_expression: scheduled_job.cron_expr.to_string(),
                    #[cfg(feature = "chrono")]
                    next_run: scheduled_job.next_run,
                    #[cfg(feature = "chrono")]
                    last_run: scheduled_job.last_run,
                    enabled: scheduled_job.enabled,
                    execution_count: scheduled_job.execution_count,
                    is_running: scheduled_job.is_running,
                });
            }
            Ok(job_infos)
        } else {
            Err(Error::concurrency("Failed to acquire jobs lock".to_string()))
        }
    }

    /// Enable or disable a specific job
    pub fn set_job_enabled(&mut self, job_id: &str, enabled: bool) -> Result<()> {
        if let Ok(mut jobs) = self.jobs.lock() {
            if let Some(job) = jobs.get_mut(job_id) {
                job.enabled = enabled;
                Ok(())
            } else {
                Err(Error::not_found(format!("Job with ID {} not found", job_id)))
            }
        } else {
            Err(Error::concurrency("Failed to acquire jobs lock".to_string()))
        }
    }

    /// Get the number of jobs in the scheduler
    pub fn job_count(&self) -> usize {
        if let Ok(jobs) = self.jobs.lock() {
            jobs.len()
        } else {
            0
        }
    }

    /// Check if the scheduler is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    /// Trigger immediate execution of a job (ignoring its schedule)
    #[cfg(feature = "tokio")]
    pub async fn trigger_job(&self, job_id: &str) -> Result<()> {
        let job = {
            if let Ok(mut jobs) = self.jobs.lock() {
                if let Some(scheduled_job) = jobs.get_mut(job_id) {
                    if scheduled_job.is_running {
                        return Err(Error::validation("Job is already running".to_string()));
                    }
                    scheduled_job.is_running = true;
                    scheduled_job.execution_count += 1;
                    scheduled_job.last_run = Some(Utc::now());
                    scheduled_job.job.clone()
                } else {
                    return Err(Error::not_found(format!("Job with ID {} not found", job_id)));
                }
            } else {
                return Err(Error::concurrency("Failed to acquire jobs lock".to_string()));
            }
        };

        let jobs_ref = self.jobs.clone();
        let job_id = job_id.to_string();
        
        tokio::spawn(async move {
            let result = job.execute().await;
            
            // Mark job as not running
            if let Ok(mut jobs_guard) = jobs_ref.lock() {
                if let Some(scheduled_job) = jobs_guard.get_mut(&job_id) {
                    scheduled_job.is_running = false;
                }
            }
            
            result
        }).await.map_err(|e| Error::custom(format!("Failed to execute job: {}", e)))?
    }

    /// Clear all jobs from the scheduler
    pub fn clear_jobs(&mut self) -> Result<()> {
        if let Ok(mut jobs) = self.jobs.lock() {
            jobs.clear();
            Ok(())
        } else {
            Err(Error::concurrency("Failed to acquire jobs lock".to_string()))
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        #[cfg(feature = "tokio")]
        {
            self.is_running.store(false, Ordering::SeqCst);
            if let Some(shutdown_tx) = self.shutdown_tx.take() {
                let _ = shutdown_tx.send(());
            }
        }
    }
}

/// Information about a scheduled job
#[derive(Debug, Clone)]
pub struct JobInfo {
    /// Job ID
    pub id: String,
    /// Job name
    pub name: String,
    /// Cron expression as string
    pub cron_expression: String,
    /// Next scheduled execution time
    #[cfg(feature = "chrono")]
    pub next_run: Option<DateTime<Utc>>,
    /// Last execution time
    #[cfg(feature = "chrono")]
    pub last_run: Option<DateTime<Utc>>,
    /// Whether the job is enabled
    pub enabled: bool,
    /// Number of times the job has been executed
    pub execution_count: u64,
    /// Whether the job is currently running
    pub is_running: bool,
}

impl fmt::Display for JobInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Job: {} ({})", self.name, self.id)?;
        writeln!(f, "  Cron: {}", self.cron_expression)?;
        writeln!(f, "  Enabled: {}", self.enabled)?;
        writeln!(f, "  Executions: {}", self.execution_count)?;
        writeln!(f, "  Running: {}", self.is_running)?;
        
        #[cfg(feature = "chrono")]
        {
            if let Some(next_run) = self.next_run {
                writeln!(f, "  Next run: {}", next_run.format("%Y-%m-%d %H:%M:%S UTC"))?;
            }
            if let Some(last_run) = self.last_run {
                writeln!(f, "  Last run: {}", last_run.format("%Y-%m-%d %H:%M:%S UTC"))?;
            }
        }
        
        Ok(())
    }
}

impl TaskHandle {
    /// Enable or disable this task
    pub fn set_enabled(&self, enabled: bool) -> Result<()> {
        if let Ok(mut jobs) = self.scheduler.lock() {
            if let Some(job) = jobs.get_mut(&self.id) {
                job.enabled = enabled;
                Ok(())
            } else {
                Err(Error::not_found(format!("Job with ID {} not found", self.id)))
            }
        } else {
            Err(Error::concurrency("Failed to acquire jobs lock".to_string()))
        }
    }

    /// Get information about this job
    pub fn get_info(&self) -> Result<JobInfo> {
        if let Ok(jobs) = self.scheduler.lock() {
            if let Some(scheduled_job) = jobs.get(&self.id) {
                Ok(JobInfo {
                    id: scheduled_job.id.clone(),
                    name: scheduled_job.job.name.clone(),
                    cron_expression: scheduled_job.cron_expr.to_string(),
                    #[cfg(feature = "chrono")]
                    next_run: scheduled_job.next_run,
                    #[cfg(feature = "chrono")]
                    last_run: scheduled_job.last_run,
                    enabled: scheduled_job.enabled,
                    execution_count: scheduled_job.execution_count,
                    is_running: scheduled_job.is_running,
                })
            } else {
                Err(Error::not_found(format!("Job with ID {} not found", self.id)))
            }
        } else {
            Err(Error::concurrency("Failed to acquire jobs lock".to_string()))
        }
    }

    /// Check if this job is currently running
    pub fn is_running(&self) -> bool {
        if let Ok(jobs) = self.scheduler.lock() {
            jobs.get(&self.id).map_or(false, |job| job.is_running)
        } else {
            false
        }
    }

    /// Get the execution count for this job
    pub fn execution_count(&self) -> u64 {
        if let Ok(jobs) = self.scheduler.lock() {
            jobs.get(&self.id).map_or(0, |job| job.execution_count)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cron::job::Job;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new();
        assert!(!scheduler.is_running());
        assert_eq!(scheduler.job_count(), 0);
    }

    #[test]
    fn test_add_job() {
        let mut scheduler = Scheduler::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let job = Job::new("test_job", Box::new(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }));
        
        let cron_expr = CronExpression::parse("* * * * *").unwrap();
        let handle = scheduler.add_job("test", job, cron_expr).unwrap();
        
        assert_eq!(scheduler.job_count(), 1);
        assert!(handle.id.starts_with("test_"));
    }

    #[test]
    fn test_remove_job() {
        let mut scheduler = Scheduler::new();
        let job = Job::new("test_job", Box::new(|| Ok(())));
        let cron_expr = CronExpression::parse("* * * * *").unwrap();
        let handle = scheduler.add_job("test", job, cron_expr).unwrap();
        
        assert_eq!(scheduler.job_count(), 1);
        
        scheduler.remove_job(&handle.id).unwrap();
        assert_eq!(scheduler.job_count(), 0);
    }

    #[test]
    fn test_job_enable_disable() {
        let mut scheduler = Scheduler::new();
        let job = Job::new("test_job", Box::new(|| Ok(())));
        let cron_expr = CronExpression::parse("* * * * *").unwrap();
        let handle = scheduler.add_job("test", job, cron_expr).unwrap();
        
        let info = handle.get_info().unwrap();
        assert!(info.enabled);
        
        handle.set_enabled(false).unwrap();
        let info = handle.get_info().unwrap();
        assert!(!info.enabled);
    }

    #[test]
    fn test_get_jobs_info() {
        let mut scheduler = Scheduler::new();
        let job = Job::new("test_job", Box::new(|| Ok(())));
        let cron_expr = CronExpression::parse("0 * * * *").unwrap();
        scheduler.add_job("test", job, cron_expr).unwrap();
        
        let jobs_info = scheduler.get_jobs_info().unwrap();
        assert_eq!(jobs_info.len(), 1);
        assert_eq!(jobs_info[0].name, "test_job");
        assert_eq!(jobs_info[0].cron_expression, "0 * * * *");
    }

    #[tokio::test]
    async fn test_scheduler_start_stop() {
        let mut scheduler = Scheduler::new();
        
        scheduler.start().await.unwrap();
        assert!(scheduler.is_running());
        
        scheduler.stop().await.unwrap();
        assert!(!scheduler.is_running());
    }

    #[tokio::test]
    async fn test_trigger_job() {
        let mut scheduler = Scheduler::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let job = Job::new("test_job", Box::new(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }));
        
        let cron_expr = CronExpression::parse("0 0 1 1 0").unwrap(); // Never runs
        let handle = scheduler.add_job("test", job, cron_expr).unwrap();
        
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        
        scheduler.trigger_job(&handle.id).await.unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        
        let info = handle.get_info().unwrap();
        assert_eq!(info.execution_count, 1);
    }

    #[test]
    fn test_scheduler_config() {
        let config = SchedulerConfig {
            tick_interval: Duration::from_millis(500),
            max_concurrent_jobs: 50,
            run_missed_jobs: false,
            timezone: "America/New_York".to_string(),
        };
        
        let scheduler = Scheduler::with_config(config.clone());
        assert_eq!(scheduler.config.tick_interval, Duration::from_millis(500));
        assert_eq!(scheduler.config.max_concurrent_jobs, 50);
        assert!(!scheduler.config.run_missed_jobs);
        assert_eq!(scheduler.config.timezone, "America/New_York");
    }

    #[test]
    fn test_clear_jobs() {
        let mut scheduler = Scheduler::new();
        let job1 = Job::new("job1", Box::new(|| Ok(())));
        let job2 = Job::new("job2", Box::new(|| Ok(())));
        let cron_expr = CronExpression::parse("* * * * *").unwrap();
        
        scheduler.add_job("test1", job1, cron_expr.clone()).unwrap();
        scheduler.add_job("test2", job2, cron_expr).unwrap();
        assert_eq!(scheduler.job_count(), 2);
        
        scheduler.clear_jobs().unwrap();
        assert_eq!(scheduler.job_count(), 0);
    }
}
