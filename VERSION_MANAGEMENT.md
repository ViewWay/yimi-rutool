# Version Management for yimi-rutool

This document provides an overview of the version management system implemented for yimi-rutool.

## 🎯 Overview

The version management system has been completely overhauled to provide:
- **Automated version releases** using cargo-release
- **Standardized commit messages** following conventional commits
- **Comprehensive CI/CD pipeline** with GitHub Actions
- **Clear branching strategy** with defined workflows
- **Quality gates** ensuring release standards

## 📁 File Structure

```
yimi-rutool/
├── .cargo-release.toml          # cargo-release configuration
├── .github/workflows/           # GitHub Actions workflows
│   ├── ci.yml                   # Continuous Integration
│   └── release.yml              # Release automation
├── scripts/
│   └── release.sh               # Release automation script
├── CHANGELOG.md                 # Unified changelog
├── CONTRIBUTING.md              # Contribution guidelines
├── BRANCH_STRATEGY.md           # Branch management strategy
├── RELEASE_CHECKLIST.md         # Release quality checklist
└── VERSION_MANAGEMENT.md        # This file
```

## 🚀 Quick Start

### Making a Release

1. **Patch Release** (bug fixes):
   ```bash
   ./scripts/release.sh patch
   ```

2. **Minor Release** (new features):
   ```bash
   ./scripts/release.sh minor
   ```

3. **Major Release** (breaking changes):
   ```bash
   ./scripts/release.sh major
   ```

4. **Dry Run** (see what would happen):
   ```bash
   ./scripts/release.sh patch --dry-run
   ```

### Running Tests Only
```bash
./scripts/release.sh test
```

## 🔧 Tools and Configuration

### cargo-release
- **Configuration**: `.cargo-release.toml`
- **Purpose**: Automated version bumping and publishing
- **Features**: 
  - Automatic version updates
  - Changelog generation
  - Git tag creation
  - crates.io publishing

### GitHub Actions
- **CI Pipeline**: `.github/workflows/ci.yml`
  - Multi-Rust version testing
  - Feature-specific testing
  - Security auditing
  - Performance benchmarking
  - Cross-platform builds

- **Release Pipeline**: `.github/workflows/release.yml`
  - Automated releases on tags
  - crates.io publishing
  - GitHub release creation
  - Asset management

### Release Script
- **Location**: `scripts/release.sh`
- **Features**:
  - Prerequisites checking
  - Automated testing
  - Version management
  - Error handling
  - Colored output

## 📝 Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `chore`: Maintenance tasks
- `ci`: CI/CD changes
- `build`: Build system changes

### Examples
```bash
feat(algorithms): add bloom filter implementation
fix(crypto): resolve RSA timing vulnerability
docs: update API documentation
chore: update dependencies
```

## 🌿 Branch Strategy

### Main Branches
- **`main`**: Production-ready code
- **`develop`**: Integration branch for features

### Supporting Branches
- **`feature/*`**: New features
- **`fix/*`**: Bug fixes
- **`hotfix/*`**: Critical production fixes
- **`release/*`**: Release preparation
- **`docs/*`**: Documentation updates
- **`refactor/*`**: Code refactoring
- **`perf/*`**: Performance improvements
- **`chore/*`**: Maintenance tasks

## 📊 Quality Gates

### Pre-Release Checks
- [ ] All tests pass
- [ ] Clippy passes with no warnings
- [ ] Code is properly formatted
- [ ] Security audit passes
- [ ] Documentation is complete
- [ ] Performance benchmarks pass

### Release Process
1. **Preparation**: Update version, changelog, documentation
2. **Testing**: Run full test suite and quality checks
3. **Release**: Use automated release script
4. **Publishing**: Automatic crates.io and GitHub release
5. **Cleanup**: Merge branches and clean up

## 🔍 Monitoring and Metrics

### CI/CD Metrics
- Build success rate
- Test execution time
- Security scan results
- Performance benchmark trends

### Release Metrics
- Time to release
- Release frequency
- Bug rate post-release
- User adoption rate

## 🛡️ Security

### Security Checks
- **cargo audit**: Vulnerability scanning
- **cargo deny**: License and security policy enforcement
- **Dependency updates**: Regular security updates
- **Code review**: All changes require review

### Security Workflow
1. Regular security audits
2. Dependency vulnerability monitoring
3. Security-focused code reviews
4. Incident response procedures

## 📚 Documentation

### Release Documentation
- **CHANGELOG.md**: Comprehensive change history
- **RELEASE_CHECKLIST.md**: Quality assurance checklist
- **CONTRIBUTING.md**: Contribution guidelines
- **BRANCH_STRATEGY.md**: Branch management guide

### API Documentation
- **docs.rs**: Automatic API documentation
- **README.md**: Project overview and examples
- **Examples**: Code examples for all features

## 🔄 Workflow Examples

### Feature Development
1. Create feature branch: `git checkout -b feature/new-feature`
2. Develop and test: `cargo test`
3. Commit with conventional format: `git commit -m "feat: add new feature"`
4. Create pull request to `develop`
5. Code review and merge
6. Delete feature branch

### Bug Fix
1. Create fix branch: `git checkout -b fix/bug-description`
2. Implement fix with tests
3. Commit: `git commit -m "fix: resolve bug in module"`
4. Create pull request to `develop`
5. Review and merge

### Release
1. Create release branch: `git checkout -b release/v0.3.0`
2. Update version and changelog
3. Run release script: `./scripts/release.sh minor`
4. Merge to main and develop
5. Clean up release branch

## 🚨 Troubleshooting

### Common Issues
- **Release fails**: Check prerequisites and test results
- **CI fails**: Review error logs and fix issues
- **Publish fails**: Verify crates.io credentials
- **Tag conflicts**: Ensure unique version numbers

### Getting Help
- Check project documentation
- Review GitHub Actions logs
- Ask in project discussions
- Contact maintainers

## 🎯 Best Practices

### Do's
- Use conventional commit messages
- Run tests before committing
- Keep branches focused and small
- Update documentation with changes
- Follow the release checklist

### Don'ts
- Don't skip quality gates
- Don't force push to shared branches
- Don't mix unrelated changes
- Don't release without testing
- Don't ignore security warnings

## 📈 Future Improvements

### Planned Enhancements
- [ ] Automated dependency updates
- [ ] Performance regression detection
- [ ] Automated security scanning
- [ ] Release notes generation
- [ ] User notification system

### Monitoring
- [ ] Release success metrics
- [ ] User feedback collection
- [ ] Performance monitoring
- [ ] Error tracking

---

This version management system ensures consistent, high-quality releases while maintaining development velocity and code quality standards.
