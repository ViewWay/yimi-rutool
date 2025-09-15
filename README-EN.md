# yimi-rutool - A Comprehensive Rust Utility Library

[![Crates.io](https://img.shields.io/crates/v/yimi-rutool.svg)](https://crates.io/crates/yimi-rutool)
[![Documentation](https://docs.rs/yimi-rutool/badge.svg)](https://docs.rs/yimi-rutool)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/ViewWay/yimi-rutool)

<div align="center">
  <h3>🌍 Language / 语言</h3>
  <p>
    <a href="README.md">中文</a> •
    <a href="README-EN.md">English</a>
  </p>
</div>

**yimi-rutool** is a comprehensive Rust utility library inspired by [Hutool](https://hutool.cn/), providing rich tool functions for daily development tasks.

## ✨ Features

- 🚀 **High Performance**: Leverages Rust's zero-cost abstractions and memory safety
- 🧰 **Feature Rich**: Covers string processing, encryption, network requests, JSON processing, and more
- 🛡️ **Type Safe**: Fully utilizes Rust's type system for code safety
- 🔧 **Modular**: Organized by functionality with optional module enabling
- 📚 **Bilingual Documentation**: Chinese and English API documentation with examples
- 🧪 **Well Tested**: 370+ unit tests with 100% pass rate
- ⚡ **Modern**: Rust 1.89 + Edition 2024, using latest language features
- 🌍 **International**: Supports Chinese and English development team collaboration

## 📦 Feature Modules

| Module | Status | Description | Features |
|--------|--------|-------------|----------|
| `core` | ✅ | Core utilities | String processing, date/time, type conversion, collection operations |
| `crypto` | ✅ | Encryption/Decryption | Symmetric/asymmetric encryption, digest algorithms, digital signatures |
| `http` | ✅ | HTTP Client | Sync/async requests, connection pooling, SSL/TLS support |
| `json` | ✅ | JSON Processing | Serialization/deserialization, JSON Path, streaming processing |
| `cache` | ✅ | Caching | Memory cache, persistent cache, LRU algorithm |
| `db` | ✅ | Database Operations | SQL execution, connection pooling, transaction management |
| `cron` | ✅ | Scheduled Tasks | Cron expression parsing, task scheduling |
| `extra` | ✅ | Extended Tools | QR code generation, image processing, compression/decompression |
| `jwt` | ✅ | JWT Authentication | Token creation/validation, multi-algorithm support, Claims management |
| `algorithms` | ✅ | Algorithm Library | Bloom filter, bitmap, various hash functions |
| `text` | ✅ | Text Processing | DFA sensitive word filtering, multiple replacement strategies, batch processing |

## 🔒 Security Notice

**Important**: The current version uses an RSA library with a known timing side-channel vulnerability ([RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071)):

- ⚠️ **Network Environment**: In network environments where attackers can observe timing information, there may be private key leakage risks
- ✅ **Local Use**: Relatively safe for local development and trusted environments
- 🔄 **Fix Status**: Official fix version not yet released, we are closely monitoring related updates

If your application involves network RSA operations, we recommend:
1. Wait for the official RSA library fix version
2. Consider using other encryption algorithms
3. Implement additional network security measures

## 🚀 Quick Start

### Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
yimi-rutool = "0.2.4"
```

### Basic Usage

```rust
use yimi_rutool::core::{StrUtil, DateUtil};

// String utilities
let result = StrUtil::is_blank("   ");
assert_eq!(result, true);

let formatted = StrUtil::format("Hello, {}!", &["World"]);
assert_eq!(formatted, "Hello, World!");

// Date and time utilities
let now = DateUtil::now();
println!("Current time: {}", now);

let tomorrow = DateUtil::offset_day(now, 1);
println!("Tomorrow: {}", tomorrow);
```

### Encryption and Decryption

```rust
use yimi_rutool::crypto::{AesUtil, Md5Util};

#[cfg(feature = "crypto")]
{
    // AES encryption
    let key = "my-secret-key-16"; // 16-byte key
    let encrypted = AesUtil::encrypt_str("Hello, World!", key)?;
    let decrypted = AesUtil::decrypt_str(&encrypted, key)?;

    assert_eq!(decrypted, "Hello, World!");

    // MD5 digest
    let hash = Md5Util::digest_hex("password");
    println!("MD5 hash: {}", hash);
}
```

### JWT Authentication

```rust
use yimi_rutool::jwt::{JwtUtil, Claims};

#[cfg(feature = "jwt")]
{
    // Create JWT Token
    let mut claims = Claims::new();
    claims.subject = Some("user123".to_string());
    claims.expires_at = Some(chrono::Utc::now().timestamp() + 3600); // Expires in 1 hour
  
    let secret = "your-secret-key";
    let token = JwtUtil::create_token(&claims, secret)?;
    println!("JWT Token: {}", token);
  
    // Validate Token
    let decoded_claims = JwtUtil::validate_token(&token, secret)?;
    println!("Subject: {:?}", decoded_claims.subject);
  
    // Create refresh token
    let refresh_token = JwtUtil::create_refresh_token("user123", secret, 24 * 7)?; // 7 days
}
```

### HTTP Requests

```rust
use yimi_rutool::http::HttpUtil;

#[cfg(feature = "http")]
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Simple GET request
    let response = HttpUtil::get("https://httpbin.org/get").await?;
    println!("Status: {}", response.status());

    // POST request
    let json_data = serde_json::json!({"key": "value"});
    let response = HttpUtil::post_json("https://httpbin.org/post", &json_data).await?;
    println!("Response: {:?}", response.json().await?);

    Ok(())
}
```

### JSON Processing

```rust
use yimi_rutool::json::JsonUtil;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

#[cfg(feature = "json")]
{
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Serialize to JSON string
    let json_str = JsonUtil::to_string(&person)?;
    println!("JSON: {}", json_str);

    // Deserialize from JSON string
    let parsed: Person = JsonUtil::from_str(&json_str)?;
    assert_eq!(parsed.name, "Alice");
    assert_eq!(parsed.age, 30);
}
```

## 🎛️ Feature Flags

Rutool uses Cargo feature flags to control which modules are enabled:

```toml
[dependencies]
yimi-rutool = { version = "0.2.4", features = ["core", "crypto"] }
```

### Available Feature Flags

- `core`: Core utilities (enabled by default)
- `crypto`: Encryption/decryption functionality
- `http`: HTTP client functionality
- `json`: JSON processing functionality
- `cache`: Caching functionality
- `db`: Database operation functionality
- `cron`: Scheduled task functionality
- `extra`: Extended tool functionality
- `algorithms`: Algorithm library functionality
- `text`: Text processing functionality
- `full`: Enable all features (default)

## 📚 Documentation

- [API Documentation](https://docs.rs/yimi-rutool) - Complete API reference
- [Usage Guide](./docs/guide.md) - Detailed usage instructions
- [Example Code](./examples/) - Practical usage examples

## 🤝 Contributing

Contributions are welcome! Please check the [Contributing Guide](./CONTRIBUTING.md) for detailed information.

### Development Environment Setup

1. Clone the repository:

```bash
git clone https://github.com/ViewWay/yimi-rutool.git
cd yimi-rutool
```

2. Run tests:

```bash
cargo test
```

3. Generate documentation:

```bash
cargo doc --open
```

4. Run benchmarks:

```bash
cargo bench
```

## 🧪 Testing

Run the complete test suite:

```bash
# Run all tests
cargo test

# Run tests for specific modules
cargo test --features crypto

# Run benchmarks
cargo bench
```

## 📝 Changelog

### v0.2.4 (2025-09-15)

#### 🛡️ Security & Quality Improvements
- **Security Audit**: Completed comprehensive security audit and addressed potential issues
- **Compiler Warnings**: Fixed useless comparison warning in `text::sensitive` module
- **Network Test Optimization**: 
  - Increased HTTP doctest timeout from 30s to 60s
  - Marked network-dependent doctests as `no_run` to avoid CI timeouts
  - Added network dependency comments

#### 🔧 Technical Improvements
- **Test Stability**: Fixed test timing issues in release mode
- **Documentation**: Created detailed security audit report (`SECURITY_AUDIT_REPORT.md`)
- **Dependencies**: Updated `serde` and `serde_json` to latest versions

#### 📊 Test Status
- **Unit Tests**: 316 tests all passing ✅
- **Documentation Tests**: 370 doctests all passing ✅
- **Compilation**: No warnings in Release and Debug modes ✅

#### ⚠️ Known Issues
- **RUSTSEC-2023-0071**: RSA timing side-channel vulnerability (medium severity, no official fix)
- **RUSTSEC-2024-0436**: paste crate no longer maintained (low severity, indirect dependency)

*Detailed security assessment and mitigation recommendations available in `SECURITY_AUDIT_REPORT.md`.*

## 📄 License

This project is licensed under **MIT OR Apache-2.0** dual license.

## 🙏 Acknowledgments

- Inspired by [Hutool](https://hutool.cn/) Java utility library
- Thanks to the Rust community and all contributors
- Thanks to all open source project support

## 📞 Contact

- Project Homepage: [https://github.com/ViewWay/yimi-rutool](https://github.com/ViewWay/yimi-rutool)
- Issue Reports: [GitHub Issues](https://github.com/ViewWay/yimi-rutool/issues)
- Email: ViewWay@example.com

---

**Making Rust development simpler and more enjoyable!** 🚀
