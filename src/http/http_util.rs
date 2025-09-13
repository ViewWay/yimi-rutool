//! HTTP utility functions
//!
//! This module provides comprehensive HTTP client utilities,
//! inspired by Hutool's HttpUtil.

use crate::error::{Error, Result};
use reqwest::{Client, Method, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// HTTP utility functions
pub struct HttpUtil;

impl HttpUtil {
    /// Create a new HTTP client with default configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = HttpUtil::client();
    ///     Ok(())
    /// }
    /// ```
    pub fn client() -> Client {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("rutool/0.1.0")
            .build()
            .unwrap()
    }

    /// Create a new HTTP client with custom timeout
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = HttpUtil::client_with_timeout(Duration::from_secs(60));
    ///     Ok(())
    /// }
    /// ```
    pub fn client_with_timeout(timeout: Duration) -> Client {
        Client::builder()
            .timeout(timeout)
            .user_agent("rutool/0.1.0")
            .build()
            .unwrap()
    }

    /// Perform a simple GET request
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = HttpUtil::get("https://httpbin.org/get").await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(url: &str) -> Result<Response> {
        let client = Self::client();
        client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a GET request and return response as text
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let text = HttpUtil::get_text("https://httpbin.org/get").await?;
    ///     println!("Response: {}", text);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_text(url: &str) -> Result<String> {
        let response = Self::get(url).await?;
        response
            .text()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a GET request and return response as JSON
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Deserialize)]
    /// struct ApiResponse {
    ///     url: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response: ApiResponse = HttpUtil::get_json("https://httpbin.org/get").await?;
    ///     println!("URL: {}", response.url);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T> {
        let response = Self::get(url).await?;
        response
            .json()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a simple POST request with JSON body
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let payload = json!({"key": "value"});
    ///     let response = HttpUtil::post_json("https://httpbin.org/post", &payload).await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn post_json<T: Serialize>(url: &str, json: &T) -> Result<Response> {
        let client = Self::client();
        client
            .post(url)
            .json(json)
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a POST request with form data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut form = HashMap::new();
    ///     form.insert("key", "value");
    ///     let response = HttpUtil::post_form("https://httpbin.org/post", &form).await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn post_form(url: &str, form: &HashMap<&str, &str>) -> Result<Response> {
        let client = Self::client();
        client
            .post(url)
            .form(form)
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a POST request with text body
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = HttpUtil::post_text("https://httpbin.org/post", "Hello, World!").await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn post_text(url: &str, text: &str) -> Result<Response> {
        let client = Self::client();
        client
            .post(url)
            .body(text.to_string())
            .header("Content-Type", "text/plain")
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a PUT request with JSON body
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let payload = json!({"key": "updated_value"});
    ///     let response = HttpUtil::put_json("https://httpbin.org/put", &payload).await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn put_json<T: Serialize>(url: &str, json: &T) -> Result<Response> {
        let client = Self::client();
        client
            .put(url)
            .json(json)
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a DELETE request
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = HttpUtil::delete("https://httpbin.org/delete").await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(url: &str) -> Result<Response> {
        let client = Self::client();
        client
            .delete(url)
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a PATCH request with JSON body
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let payload = json!({"key": "patched_value"});
    ///     let response = HttpUtil::patch_json("https://httpbin.org/patch", &payload).await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn patch_json<T: Serialize>(url: &str, json: &T) -> Result<Response> {
        let client = Self::client();
        client
            .patch(url)
            .json(json)
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a HEAD request
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = HttpUtil::head("https://httpbin.org/get").await?;
    ///     println!("Status: {}", response.status());
    ///     println!("Content-Length: {:?}", response.headers().get("content-length"));
    ///     Ok(())
    /// }
    /// ```
    pub async fn head(url: &str) -> Result<Response> {
        let client = Self::client();
        client
            .head(url)
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Perform a request with custom method and headers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use reqwest::Method;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut headers = HashMap::new();
    ///     headers.insert("Authorization", "Bearer token123");
    ///     headers.insert("Custom-Header", "custom-value");
    ///     
    ///     let response = HttpUtil::request(
    ///         Method::GET,
    ///         "https://httpbin.org/headers",
    ///         Some(&headers),
    ///         None::<&()>
    ///     ).await?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub async fn request<T: Serialize>(
        method: Method,
        url: &str,
        headers: Option<&HashMap<&str, &str>>,
        body: Option<&T>,
    ) -> Result<Response> {
        let client = Self::client();
        let mut request = client.request(method, url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(*key, *value);
            }
        }

        if let Some(body) = body {
            request = request.json(body);
        }

        request
            .send()
            .await
            .map_err(|e| Error::Http(e))
    }

    /// Download a file from URL to local path
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     HttpUtil::download_file(
    ///         "https://httpbin.org/json",
    ///         "/tmp/downloaded.json"
    ///     ).await?;
    ///     println!("File downloaded successfully");
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_file(url: &str, path: &str) -> Result<()> {
        let response = Self::get(url).await?;
        let bytes = response.bytes().await.map_err(|e| Error::Http(e))?;
        
        let mut file = File::create(path).await.map_err(|e| Error::Io(e))?;
        file.write_all(&bytes).await.map_err(|e| Error::Io(e))?;
        
        Ok(())
    }

    /// Check if a URL is reachable (returns 2xx status code)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let is_reachable = HttpUtil::is_reachable("https://httpbin.org/get").await?;
    ///     println!("URL reachable: {}", is_reachable);
    ///     Ok(())
    /// }
    /// ```
    pub async fn is_reachable(url: &str) -> Result<bool> {
        match Self::head(url).await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Get HTTP status code for a URL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let status = HttpUtil::get_status("https://httpbin.org/status/404").await?;
    ///     println!("Status code: {}", status);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_status(url: &str) -> Result<StatusCode> {
        let response = Self::head(url).await?;
        Ok(response.status())
    }

    /// Get response headers for a URL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let headers = HttpUtil::get_headers("https://httpbin.org/get").await?;
    ///     for (name, value) in &headers {
    ///         println!("{}: {}", name, value);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_headers(url: &str) -> Result<HashMap<String, String>> {
        let response = Self::head(url).await?;
        let mut headers = HashMap::new();
        
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.to_string(), value_str.to_string());
            }
        }
        
        Ok(headers)
    }

    /// Perform multiple concurrent GET requests
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let urls = vec![
    ///         "https://httpbin.org/get",
    ///         "https://httpbin.org/headers",
    ///         "https://httpbin.org/user-agent",
    ///     ];
    ///     let responses = HttpUtil::get_multiple(&urls).await?;
    ///     println!("Fetched {} responses", responses.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_multiple(urls: &[&str]) -> Result<Vec<Response>> {
        let client = Self::client();
        let futures: Vec<_> = urls.iter()
            .map(|url| client.get(*url).send())
            .collect();
        
        let results = futures::future::join_all(futures).await;
        let mut responses = Vec::new();
        
        for result in results {
            responses.push(result.map_err(|e| Error::Http(e))?);
        }
        
        Ok(responses)
    }

    /// Build a query string from parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("key1", "value1");
    /// params.insert("key2", "value with spaces");
    /// 
    /// let query = HttpUtil::build_query_string(&params);
    /// println!("Query string: {}", query);
    /// ```
    pub fn build_query_string(params: &HashMap<&str, &str>) -> String {
        use urlencoding::encode;
        
        params.iter()
            .map(|(key, value)| format!("{}={}", encode(key), encode(value)))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// Parse query string into parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// let query = "key1=value1&key2=value%20with%20spaces";
    /// let params = HttpUtil::parse_query_string(query);
    /// 
    /// assert_eq!(params.get("key1"), Some(&"value1".to_string()));
    /// assert_eq!(params.get("key2"), Some(&"value with spaces".to_string()));
    /// ```
    pub fn parse_query_string(query: &str) -> HashMap<String, String> {
        use urlencoding::decode;
        
        let mut params = HashMap::new();
        
        for pair in query.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                if let (Ok(decoded_key), Ok(decoded_value)) = (decode(key), decode(value)) {
                    params.insert(decoded_key.to_string(), decoded_value.to_string());
                }
            }
        }
        
        params
    }

    /// Build a URL with query parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("q", "rust programming");
    /// params.insert("page", "1");
    /// 
    /// let url = HttpUtil::build_url("https://example.com/search", &params);
    /// println!("URL: {}", url);
    /// ```
    pub fn build_url(base_url: &str, params: &HashMap<&str, &str>) -> String {
        if params.is_empty() {
            return base_url.to_string();
        }
        
        let query = Self::build_query_string(params);
        let separator = if base_url.contains('?') { "&" } else { "?" };
        format!("{}{}{}", base_url, separator, query)
    }

    /// Extract domain from URL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// let domain = HttpUtil::extract_domain("https://www.example.com/path?query=value");
    /// assert_eq!(domain, Some("www.example.com".to_string()));
    /// ```
    pub fn extract_domain(url: &str) -> Option<String> {
        if let Ok(parsed) = Url::parse(url) {
            parsed.host_str().map(|s| s.to_string())
        } else {
            None
        }
    }

    /// Check if URL is valid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// assert!(HttpUtil::is_valid_url("https://www.example.com"));
    /// assert!(!HttpUtil::is_valid_url("not-a-url"));
    /// ```
    pub fn is_valid_url(url: &str) -> bool {
        Url::parse(url).is_ok()
    }
}

// Blocking HTTP utilities for synchronous code
impl HttpUtil {
    /// Perform a blocking GET request
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = HttpUtil::get_blocking("https://httpbin.org/get")?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub fn get_blocking(url: &str) -> Result<reqwest::blocking::Response> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("rutool/0.1.0")
            .build()
            .map_err(|e| Error::Http(e))?;
        
        client
            .get(url)
            .send()
            .map_err(|e| Error::Http(e))
    }

    /// Perform a blocking GET request and return response as text
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let text = HttpUtil::get_text_blocking("https://httpbin.org/get")?;
    ///     println!("Response: {}", text);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_text_blocking(url: &str) -> Result<String> {
        let response = Self::get_blocking(url)?;
        response
            .text()
            .map_err(|e| Error::Http(e))
    }

    /// Perform a blocking POST request with JSON body
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::http::HttpUtil;
    /// use serde_json::json;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let payload = json!({"key": "value"});
    ///     let response = HttpUtil::post_json_blocking("https://httpbin.org/post", &payload)?;
    ///     println!("Status: {}", response.status());
    ///     Ok(())
    /// }
    /// ```
    pub fn post_json_blocking<T: Serialize>(url: &str, json: &T) -> Result<reqwest::blocking::Response> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("rutool/0.1.0")
            .build()
            .map_err(|e| Error::Http(e))?;
        
        client
            .post(url)
            .json(json)
            .send()
            .map_err(|e| Error::Http(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_build_query_string() {
        let mut params = HashMap::new();
        params.insert("key1", "value1");
        params.insert("key2", "value with spaces");
        
        let query = HttpUtil::build_query_string(&params);
        assert!(query.contains("key1=value1"));
        assert!(query.contains("key2=value%20with%20spaces"));
    }

    #[test]
    fn test_parse_query_string() {
        let query = "key1=value1&key2=value%20with%20spaces";
        let params = HttpUtil::parse_query_string(query);
        
        assert_eq!(params.get("key1"), Some(&"value1".to_string()));
        assert_eq!(params.get("key2"), Some(&"value with spaces".to_string()));
    }

    #[test]
    fn test_build_url() {
        let mut params = HashMap::new();
        params.insert("q", "rust programming");
        params.insert("page", "1");
        
        let url = HttpUtil::build_url("https://example.com/search", &params);
        assert!(url.starts_with("https://example.com/search?"));
        assert!(url.contains("q=rust%20programming"));
        assert!(url.contains("page=1"));
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            HttpUtil::extract_domain("https://www.example.com/path?query=value"),
            Some("www.example.com".to_string())
        );
        assert_eq!(
            HttpUtil::extract_domain("http://localhost:8080/api"),
            Some("localhost".to_string())
        );
        assert_eq!(HttpUtil::extract_domain("invalid-url"), None);
    }

    #[test]
    fn test_is_valid_url() {
        assert!(HttpUtil::is_valid_url("https://www.example.com"));
        assert!(HttpUtil::is_valid_url("http://localhost:8080"));
        assert!(HttpUtil::is_valid_url("ftp://files.example.com"));
        assert!(!HttpUtil::is_valid_url("not-a-url"));
        assert!(!HttpUtil::is_valid_url(""));
    }

    #[test]
    fn test_client_creation() {
        let _client = HttpUtil::client();
        // Just test that we can create a client without panicking
        // Note: reqwest::Client doesn't expose user_agent() method
        // We can't directly test the user agent, but we can test the client is created
        assert!(true); // Client creation succeeded if we reach here
    }

    #[test]
    fn test_client_with_timeout() {
        let timeout = Duration::from_secs(60);
        let _client = HttpUtil::client_with_timeout(timeout);
        // Just test that we can create a client with custom timeout
        assert!(true); // Client creation succeeded if we reach here
    }

    // Integration tests that require internet connection
    #[cfg(feature = "integration_tests")]
    mod integration_tests {
        use super::*;

        #[tokio::test]
        async fn test_get_request() {
            let response = HttpUtil::get("https://httpbin.org/get").await.unwrap();
            assert!(response.status().is_success());
        }

        #[tokio::test]
        async fn test_get_text() {
            let text = HttpUtil::get_text("https://httpbin.org/get").await.unwrap();
            assert!(!text.is_empty());
        }

        #[tokio::test]
        async fn test_post_json() {
            use serde_json::json;
            let payload = json!({"key": "test_value"});
            let response = HttpUtil::post_json("https://httpbin.org/post", &payload).await.unwrap();
            assert!(response.status().is_success());
        }

        #[tokio::test]
        async fn test_is_reachable() {
            let reachable = HttpUtil::is_reachable("https://httpbin.org/get").await.unwrap();
            assert!(reachable);
            
            let unreachable = HttpUtil::is_reachable("https://invalid-domain-12345.com").await.unwrap();
            assert!(!unreachable);
        }

        #[test]
        fn test_blocking_get() {
            let response = HttpUtil::get_blocking("https://httpbin.org/get").unwrap();
            assert!(response.status().is_success());
        }
    }
}
