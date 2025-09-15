# Changelog

<div align="center">
  <h3>🌍 Language / 语言</h3>
  <p>
    <a href="CHANGELOG.md">中文</a> •
    <a href="CHANGELOG-EN.md">English</a>
  </p>
</div>

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Version management automation tools and scripts
- GitHub Actions CI/CD workflows
- Unified version release process

### Changed
- Improved version management strategy
- Standardized commit message format

## [0.2.4] - 2025-09-15

### 🛡️ Security & Quality Improvements
- **Security Audit**: Completed comprehensive security audit and addressed potential issues
- **Compiler Warnings**: Fixed useless comparison warning in `text::sensitive` module
- **Network Test Optimization**: 
  - Increased HTTP doctest timeout from 30s to 60s
  - Marked network-dependent doctests as `no_run` to avoid CI timeouts
  - Added network dependency comments

### 🔧 Technical Improvements
- **Test Stability**: Fixed test timing issues in release mode
- **Documentation**: Created detailed security audit report (`SECURITY_AUDIT_REPORT.md`)
- **Dependencies**: Updated `serde` and `serde_json` to latest versions

### 📊 Test Status
- **Unit Tests**: 316 tests all passing ✅
- **Documentation Tests**: 370 doctests all passing ✅
- **Compilation**: No warnings in Release and Debug modes ✅

### ⚠️ Known Issues
- **RUSTSEC-2023-0071**: RSA timing side-channel vulnerability (medium severity, no official fix)
- **RUSTSEC-2024-0436**: paste crate no longer maintained (low severity, indirect dependency)

*Detailed security assessment and mitigation recommendations available in `SECURITY_AUDIT_REPORT.md`.*

## [0.2.3] - 2025-09-12

### 🎉 Major Feature Release

#### New Modules

##### 🔍 Algorithms Module - High-Performance Algorithm Library
- **Bloom Filter**
  - Standard Bloom Filter: Efficient set membership testing
  - Counting Bloom Filter: Support for element deletion
  - Auto parameter optimization: Calculate optimal parameters based on expected items and false positive rate
  - Multiple hash functions: MurmurHash3, FNV-1a, SipHash, etc.
  - Builder pattern: Flexible configuration interface
  - Performance benchmarks: Thoroughly tested and optimized

- **BitMap Tools**
  - Efficient bit operations: Set, get, flip bits
  - Bitwise operations: AND, OR, XOR, NOT operations
  - Statistics: Count 1/0 bits
  - Iterators: Traverse set/unset bits
  - Dynamic sizing: Support runtime size adjustment

- **Hash Functions Library**
  - Multiple hash algorithms: Support for different hash functions
  - Double hashing: Generate multiple independent hash values
  - Quality assessment: Hash function distribution uniformity testing
  - Performance optimization: Optimized implementations for different use cases

##### 📝 Text Module - Intelligent Text Processing
- **DFA Sensitive Word Filter**
  - Aho-Corasick algorithm: Efficient multi-pattern matching
  - Deterministic Finite Automaton: O(n) time complexity text scanning
  - Multiple replacement strategies:
    - Mask replacement (****)
    - Custom string replacement
    - Character repetition replacement
    - Highlight marking
    - Complete deletion
  - Flexible configuration:
    - Case sensitive/insensitive
    - Custom sensitive word library
    - File loading support
  - Performance monitoring: Built-in processing statistics and performance analysis
  - Builder pattern: Clean API design

### 🔧 Technical Improvements

#### Code Quality
- **Test Coverage**: Added 70+ unit tests, total 390+ tests
- **Documentation Tests**: Added 100+ documentation example tests
- **Code Lines**: Project total code reached 163,642 lines (+16%)
- **Performance Optimization**: Release mode performance tests passed

#### Security Enhancements
- **Security Audit**: Integrated cargo-audit security checks
- **Vulnerability Handling**: 
  - Identified and documented RUSTSEC-2023-0071 RSA timing attack vulnerability
  - Added security usage guidelines and risk descriptions
  - Clearly documented network environment usage risks

#### API Design
- **Type Safety**: All new APIs support `?Sized` types, compatible with `&str` and other unsized types
- **Error Handling**: Unified error types and handling mechanisms
- **Memory Efficiency**: Zero-copy and efficient memory usage patterns

### 📊 Project Statistics

| Metric | v0.2.2 | v0.2.3 | Growth |
|--------|--------|--------|--------|
| Module Count | 9 | 11 | +22% |
| Test Count | 321 | 390+ | +21% |
| Code Lines | ~140k | 163k | +16% |
| Features | Basic Tools | Algorithms+Text | Advanced Features |

### 🚀 Performance Characteristics

#### Bloom Filter Performance
- **Memory Efficiency**: 60-80% memory savings compared to hash tables
- **Query Speed**: O(k) time complexity, k is number of hash functions
- **False Positive Control**: Can precisely control within theoretical range

#### DFA Text Filtering Performance
- **Time Complexity**: O(n), n is text length
- **Concurrency Safe**: Supports multi-threaded concurrent access
- **Memory Usage**: Efficient state machine implementation, optimal memory usage

### 🔄 Version Compatibility

- **Backward Compatible**: All existing APIs remain compatible
- **New Features**: Controlled through feature flags, no impact on existing users
- **Dependency Updates**: Maintain stable dependency versions

### 🛡️ Security Notice

⚠️ **Important**: This version uses RSA library with known security vulnerability RUSTSEC-2023-0071:
- Potential timing side-channel attack risk in network environments
- Relatively safe for local development and trusted environments
- Recommend monitoring RSA library official fix updates

## [0.2.2] - 2025-09-10

### 📚 Documentation Updates
- Comprehensive project documentation updates
- Enhanced API documentation with examples
- Improved README with better structure

## [0.2.1] - 2025-09-08

### 📚 Documentation Updates
- Updated project documentation milestone
- Enhanced code quality optimization
- Bilingual documentation improvements

### 🔧 Code Quality
- Continuous code quality optimization
- Clippy-friendly version improvements
- Performance enhancements

## [0.2.0] - 2025-09-05

### 🎯 JWT Module Implementation
- Major release with JWT authentication module
- Token creation and validation
- Multiple algorithm support
- Claims management

## [0.1.2] - 2025-08-30

### 🛠️ Bug Fixes
- Fixed documentation warnings
- Updated naming references to yimi-rutool
- Prepared for crates.io publication

## [0.1.1] - 2025-08-25

### 🛠️ Bug Fixes
- Fixed documentation warnings
- Initial project setup improvements

## [0.1.0] - 2025-08-20

### 🚀 Initial Release
- Core utility modules
- Basic functionality implementation
- Initial project structure

---

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version when you make incompatible API changes
- **MINOR** version when you add functionality in a backwards compatible manner
- **PATCH** version when you make backwards compatible bug fixes

## Types of Changes

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** for vulnerability fixes
