# yimi-rutool - A Comprehensive Rust Utility Library

[![Crates.io](https://img.shields.io/crates/v/yimi-rutool.svg)](https://crates.io/crates/yimi-rutool)
[![Documentation](https://docs.rs/yimi-rutool/badge.svg)](https://docs.rs/yimi-rutool)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/yimiliya/yimi-rutool)

**yimi-rutool** 是一个受 [Hutool](https://hutool.cn/) 启发的综合性 Rust 工具库，为日常开发任务提供丰富的工具函数。

## ✨ 特性

- 🚀 **高性能**: 利用 Rust 的零成本抽象和内存安全特性
- 🧰 **功能丰富**: 涵盖字符串处理、加密、网络请求、JSON处理等多个领域
- 🛡️ **类型安全**: 充分利用 Rust 类型系统保证代码安全性
- 🔧 **模块化**: 按功能划分模块，可选择性启用
- 📚 **文档完善**: 详细的 API 文档和使用示例
- 🧪 **测试充分**: 完善的单元测试和集成测试

## 📦 功能模块

| 模块 | 描述 | 功能特性 |
|------|------|----------|
| `core` | 核心工具类 | 字符串处理、日期时间、类型转换、集合操作 |
| `crypto` | 加密解密 | 对称/非对称加密、摘要算法、数字签名 |
| `http` | HTTP客户端 | 同步/异步请求、连接池、SSL/TLS支持 |
| `json` | JSON处理 | 序列化/反序列化、JSON Path、流式处理 |
| `cache` | 缓存 | 内存缓存、持久化缓存、LRU算法 |
| `db` | 数据库操作 | SQL执行、连接池、事务管理 |
| `cron` | 定时任务 | Cron表达式解析、任务调度 |
| `extra` | 扩展工具 | 二维码生成、图片处理、压缩解压 |

## 🚀 快速开始

### 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
yimi-rutool = "0.1"
```

### 基础用法

```rust
use yimi_rutool::core::{StrUtil, DateUtil};

// 字符串工具
let result = StrUtil::is_blank("   ");
assert_eq!(result, true);

let formatted = StrUtil::format("Hello, {}!", &["World"]);
assert_eq!(formatted, "Hello, World!");

// 日期时间工具
let now = DateUtil::now();
println!("Current time: {}", now);

let tomorrow = DateUtil::offset_day(now, 1);
println!("Tomorrow: {}", tomorrow);
```

### 加密解密

```rust
use yimi_rutool::crypto::{AesUtil, Md5Util};

#[cfg(feature = "crypto")]
{
    // AES 加密
    let key = "my-secret-key-16"; // 16字节密钥
    let encrypted = AesUtil::encrypt_str("Hello, World!", key)?;
    let decrypted = AesUtil::decrypt_str(&encrypted, key)?;

    assert_eq!(decrypted, "Hello, World!");

    // MD5 摘要
    let hash = Md5Util::digest_hex("password");
    println!("MD5 hash: {}", hash);
}
```

### HTTP 请求

```rust
use yimi_rutool::http::HttpUtil;

#[cfg(feature = "http")]
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // 简单的 GET 请求
    let response = HttpUtil::get("https://httpbin.org/get").await?;
    println!("Status: {}", response.status());

    // POST 请求
    let json_data = serde_json::json!({"key": "value"});
    let response = HttpUtil::post_json("https://httpbin.org/post", &json_data).await?;
    println!("Response: {:?}", response.json().await?);

    Ok(())
}
```

### JSON 处理

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

    // 序列化为 JSON 字符串
    let json_str = JsonUtil::to_string(&person)?;
    println!("JSON: {}", json_str);

    // 从 JSON 字符串反序列化
    let parsed: Person = JsonUtil::from_str(&json_str)?;
    assert_eq!(parsed.name, "Alice");
    assert_eq!(parsed.age, 30);
}
```

## 🎛️ 功能开关

Rutool 使用 Cargo 的功能标志来控制启用哪些模块：

```toml
[dependencies]
yimi-rutool = { version = "0.1", features = ["core", "crypto"] }
```

### 可用功能标志

- `core`: 核心工具类（默认启用）
- `crypto`: 加密解密功能
- `http`: HTTP 客户端功能
- `json`: JSON 处理功能
- `cache`: 缓存功能
- `db`: 数据库操作功能
- `cron`: 定时任务功能
- `extra`: 扩展工具功能
- `full`: 启用所有功能（默认）

## 📚 详细文档

- [API 文档](https://docs.rs/yimi-rutool) - 完整的 API 参考
- [使用指南](./docs/guide.md) - 详细的使用说明
- [示例代码](./examples/) - 实际使用示例

## 🤝 贡献

欢迎贡献代码！请查看 [贡献指南](./CONTRIBUTING.md) 了解详细信息。

### 开发环境设置

1. 克隆仓库：
```bash
git clone https://github.com/yimiliya/yimi-rutool.git
cd yimi-rutool
```

2. 运行测试：
```bash
cargo test
```

3. 生成文档：
```bash
cargo doc --open
```

4. 运行基准测试：
```bash
cargo bench
```

## 🧪 测试

运行完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test --features crypto

# 运行基准测试
cargo bench
```

## 📄 许可证

本项目采用 **MIT OR Apache-2.0** 双许可证。

## 🙏 致谢

- 受 [Hutool](https://hutool.cn/) Java 工具库启发
- 感谢 Rust 社区和所有贡献者
- 感谢所有开源项目的支持

## 📞 联系方式

- 项目主页: [https://github.com/yimiliya/yimi-rutool](https://github.com/yimiliya/yimi-rutool)
- 问题反馈: [GitHub Issues](https://github.com/yimiliya/yimi-rutool/issues)
- 邮箱: yimiliya@example.com

---

**让 Rust 开发变得更加简单和愉快！** 🚀
