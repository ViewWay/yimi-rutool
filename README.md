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
- 📚 **双语文档**: 中英文双语API文档和使用示例
- 🧪 **测试充分**: 250+ 单元测试，100%通过率
- ⚡ **现代化**: Rust 1.89 + Edition 2024，使用最新语言特性
- 🌍 **国际化**: 支持中英文开发团队协作

## 📦 功能模块

| 模块         | 状态 | 描述       | 功能特性                                 |
| ------------ | ---- | ---------- | ---------------------------------------- |
| `core`     | ✅   | 核心工具类 | 字符串处理、日期时间、类型转换、集合操作 |
| `crypto`   | ✅   | 加密解密   | 对称/非对称加密、摘要算法、数字签名      |
| `http`     | ✅   | HTTP客户端 | 同步/异步请求、连接池、SSL/TLS支持       |
| `json`     | ✅   | JSON处理   | 序列化/反序列化、JSON Path、流式处理     |
| `cache`    | ✅   | 缓存       | 内存缓存、持久化缓存、LRU算法            |
| `db`       | ✅   | 数据库操作 | SQL执行、连接池、事务管理                |
| `cron`     | ✅   | 定时任务   | Cron表达式解析、任务调度                 |
| `extra`    | ✅   | 扩展工具   | 二维码生成、图片处理、压缩解压           |
| `jwt`      | ✅   | JWT认证    | 令牌创建/验证、多算法支持、Claims管理    |
| `algorithms` | ✅ | 算法库     | 布隆过滤器、位图、多种哈希函数           |
| `text`     | ✅   | 文本处理   | DFA敏感词过滤、多种替换策略、批量处理    |

## 🔒 安全说明

**重要**: 当前版本使用的RSA库存在已知的时序侧信道漏洞 ([RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071))：

- ⚠️ **网络环境**: 在可被攻击者观察到时序信息的网络环境中，可能存在私钥泄露风险
- ✅ **本地使用**: 在本地开发和受信任环境中使用相对安全
- 🔄 **修复状态**: 官方尚未发布修复版本，我们正在密切关注相关更新

如果你的应用场景涉及网络RSA操作，建议：
1. 等待RSA库官方修复版本
2. 考虑使用其他加密算法
3. 实施额外的网络安全措施

## 🚀 快速开始

### 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
yimi-rutool = "0.2.4"
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

### JWT 认证

```rust
use yimi_rutool::jwt::{JwtUtil, Claims};

#[cfg(feature = "jwt")]
{
    // 创建 JWT Token
    let mut claims = Claims::new();
    claims.subject = Some("user123".to_string());
    claims.expires_at = Some(chrono::Utc::now().timestamp() + 3600); // 1小时后过期
  
    let secret = "your-secret-key";
    let token = JwtUtil::create_token(&claims, secret)?;
    println!("JWT Token: {}", token);
  
    // 验证 Token
    let decoded_claims = JwtUtil::validate_token(&token, secret)?;
    println!("Subject: {:?}", decoded_claims.subject);
  
    // 创建刷新令牌
    let refresh_token = JwtUtil::create_refresh_token("user123", secret, 24 * 7)?; // 7天
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
git clone https://github.com/ViewWay/yimi-rutool.git
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

## 📝 更新记录

### v0.2.4 (2025-09-15)

#### 🛡️ 安全和质量改进
- **代码审计**: 完成全面安全审计，发现并处理潜在问题
- **编译警告修复**: 修复`text::sensitive`模块中的无用比较警告
- **网络测试优化**: 
  - HTTP doctest超时时间从30秒增加到60秒
  - 将依赖网络的doctest标记为`no_run`避免CI环境超时
  - 添加网络依赖说明注释

#### 🔧 技术改进  
- **测试稳定性**: 修复release模式下的测试时序问题
- **文档完善**: 创建详细的安全审计报告(`SECURITY_AUDIT_REPORT.md`)
- **依赖更新**: 更新`serde`和`serde_json`到最新版本

#### 📊 测试状态
- **单元测试**: 316个测试全部通过 ✅
- **文档测试**: 370个doctest全部通过 ✅  
- **编译检查**: Release和Debug模式编译无警告 ✅

#### ⚠️ 已知问题
- **RUSTSEC-2023-0071**: RSA时序侧信道漏洞（中危，官方未修复）
- **RUSTSEC-2024-0436**: paste crate不再维护（低危，间接依赖）

*详细的安全评估和缓解建议请参考项目根目录的`SECURITY_AUDIT_REPORT.md`文件。*

## 📄 许可证

本项目采用 **MIT OR Apache-2.0** 双许可证。

## 🙏 致谢

- 受 [Hutool](https://hutool.cn/) Java 工具库启发
- 感谢 Rust 社区和所有贡献者
- 感谢所有开源项目的支持

## 📞 联系方式

- 项目主页: [https://github.com/ViewWay/yimi-rutool](https://github.com/ViewWay/yimi-rutool)
- 问题反馈: [GitHub Issues](https://github.com/ViewWay/yimi-rutool/issues)
- 邮箱: ViewWay@example.com

---

**让 Rust 开发变得更加简单和愉快！** 🚀
