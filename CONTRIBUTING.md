# Contributing to yimi-rutool

Thank you for your interest in contributing to yimi-rutool! This document provides guidelines and information for contributors.

## 🚀 Quick Start

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and ensure they pass
5. Submit a pull request

## 📝 Commit Message Convention

We use [Conventional Commits](https://www.conventionalcommits.org/) to maintain a clear and consistent commit history.

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools and libraries such as documentation generation
- **ci**: Changes to our CI configuration files and scripts
- **build**: Changes that affect the build system or external dependencies
- **revert**: Reverts a previous commit

### Scopes (Optional)

- **core**: Core utility functions
- **crypto**: Cryptographic functions
- **http**: HTTP client functionality
- **json**: JSON processing
- **cache**: Caching functionality
- **db**: Database operations
- **cron**: Cron job scheduling
- **extra**: Extra utilities
- **jwt**: JWT authentication
- **algorithms**: Algorithm implementations
- **text**: Text processing
- **deps**: Dependencies
- **ci**: Continuous Integration
- **docs**: Documentation

### Examples

```bash
# Feature
feat(algorithms): add bloom filter implementation

# Bug fix
fix(crypto): resolve RSA timing vulnerability

# Documentation
docs: update API documentation for text module

# Performance improvement
perf(cache): optimize LRU cache performance

# Breaking change
feat(http)!: change default timeout to 60 seconds

BREAKING CHANGE: HTTP client default timeout changed from 30s to 60s
```

### Breaking Changes

Use `!` after the type/scope to indicate breaking changes:

```
feat(api)!: change function signature

BREAKING CHANGE: The function now requires an additional parameter
```

## 🌿 Branch Strategy

### Branch Naming

- **feature/**: New features
  - `feature/bloom-filter`
  - `feature/text-processing`
- **fix/**: Bug fixes
  - `fix/rsa-vulnerability`
  - `fix/memory-leak`
- **docs/**: Documentation updates
  - `docs/api-reference`
  - `docs/examples`
- **refactor/**: Code refactoring
  - `refactor/cache-module`
  - `refactor/error-handling`
- **perf/**: Performance improvements
  - `perf/hash-functions`
  - `perf/memory-usage`
- **chore/**: Maintenance tasks
  - `chore/update-dependencies`
  - `chore/ci-improvements`

### Branch Workflow

1. **main**: Production-ready code
2. **develop**: Integration branch for features
3. **feature/***: Feature development branches
4. **fix/***: Bug fix branches
5. **release/***: Release preparation branches

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test --features crypto

# Run documentation tests
cargo test --doc

# Run with verbose output
cargo test -- --nocapture
```

### Test Requirements

- All new features must include tests
- Test coverage should be maintained or improved
- Documentation examples must be tested
- Performance tests for critical paths

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
    }

    #[test]
    #[should_panic]
    fn test_invalid_input() {
        // Test error cases
    }
}
```

## 📚 Documentation

### Code Documentation

- All public APIs must be documented
- Use `///` for documentation comments
- Include examples in documentation
- Document error conditions

```rust
/// Calculates the MD5 hash of the input string.
///
/// # Arguments
///
/// * `input` - The string to hash
///
/// # Returns
///
/// Returns the MD5 hash as a hexadecimal string.
///
/// # Examples
///
/// ```
/// use yimi_rutool::crypto::Md5Util;
///
/// let hash = Md5Util::digest_hex("hello world");
/// assert_eq!(hash, "5d41402abc4b2a76b9719d911017c592");
/// ```
pub fn digest_hex(input: &str) -> String {
    // Implementation
}
```

### README Updates

- Update README.md for new features
- Add examples for new functionality
- Update version numbers
- Update feature matrix

## 🔍 Code Review Process

### Before Submitting

1. Run `cargo fmt` to format code
2. Run `cargo clippy` to check for issues
3. Run `cargo test` to ensure tests pass
4. Update documentation if needed
5. Update CHANGELOG.md if applicable

### Pull Request Guidelines

- Provide a clear description of changes
- Reference related issues
- Include test results
- Update documentation
- Follow the commit message convention

### Review Checklist

- [ ] Code follows Rust conventions
- [ ] Tests are included and pass
- [ ] Documentation is updated
- [ ] No breaking changes without notice
- [ ] Performance impact is considered
- [ ] Security implications are reviewed

## 🚀 Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality in a backwards compatible manner
- **PATCH**: Backwards compatible bug fixes

### Release Checklist

- [ ] All tests pass
- [ ] Documentation is up to date
- [ ] CHANGELOG.md is updated
- [ ] Version numbers are updated
- [ ] Security audit is clean
- [ ] Performance benchmarks pass

### Automated Release

Use the release script for automated releases:

```bash
# Patch release (0.1.0 -> 0.1.1)
./scripts/release.sh patch

# Minor release (0.1.0 -> 0.2.0)
./scripts/release.sh minor

# Major release (0.1.0 -> 1.0.0)
./scripts/release.sh major

# Dry run to see what would happen
./scripts/release.sh patch --dry-run
```

## 🛡️ Security

### Security Issues

- Report security issues privately to maintainers
- Do not create public issues for security vulnerabilities
- Follow responsible disclosure practices

### Security Guidelines

- Use safe Rust practices
- Avoid unsafe code unless absolutely necessary
- Validate all inputs
- Handle errors appropriately
- Use secure random number generation
- Follow cryptographic best practices

## 📞 Getting Help

- **Issues**: Use GitHub Issues for bug reports and feature requests
- **Discussions**: Use GitHub Discussions for questions and general discussion
- **Email**: Contact maintainers for security issues

## 📄 License

By contributing to yimi-rutool, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

## 🙏 Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to yimi-rutool! 🚀
