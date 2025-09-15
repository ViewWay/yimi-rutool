# Release Checklist for yimi-rutool

This checklist ensures that every release meets quality standards and follows best practices.

## 📋 Pre-Release Checklist

### 🔍 Code Quality
- [ ] All tests pass (`cargo test`)
- [ ] Documentation tests pass (`cargo test --doc`)
- [ ] Clippy passes with no warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --all -- --check`)
- [ ] No unused dependencies (`cargo machete`)
- [ ] No dead code warnings
- [ ] All public APIs are documented
- [ ] Examples in documentation are tested

### 🛡️ Security
- [ ] Security audit passes (`cargo audit`)
- [ ] No known vulnerabilities
- [ ] Dependencies are up to date
- [ ] No hardcoded secrets or credentials
- [ ] Input validation is implemented
- [ ] Error handling is appropriate
- [ ] Memory safety is ensured

### 📚 Documentation
- [ ] README.md is updated
- [ ] CHANGELOG.md is updated with new features/fixes
- [ ] API documentation is complete
- [ ] Examples are provided for new features
- [ ] Breaking changes are documented
- [ ] Migration guide is provided (if needed)
- [ ] Version numbers are updated

### 🧪 Testing
- [ ] Unit tests cover new functionality
- [ ] Integration tests pass
- [ ] Performance tests pass
- [ ] Edge cases are tested
- [ ] Error conditions are tested
- [ ] Cross-platform compatibility verified
- [ ] Feature flag combinations tested

### 🔧 Configuration
- [ ] Cargo.toml version is updated
- [ ] Dependencies are properly specified
- [ ] Feature flags are correctly configured
- [ ] Build configuration is optimized
- [ ] CI/CD configuration is updated

## 🚀 Release Process

### 1. Pre-Release Preparation
- [ ] Create release branch from `develop`
- [ ] Update version numbers in all relevant files
- [ ] Update CHANGELOG.md with release notes
- [ ] Run full test suite
- [ ] Perform security audit
- [ ] Update documentation

### 2. Release Execution
- [ ] Use release script: `./scripts/release.sh <version-type>`
- [ ] Verify version bump is correct
- [ ] Confirm all tests pass
- [ ] Check that CI/CD pipeline passes
- [ ] Create and push git tag
- [ ] Publish to crates.io (if applicable)

### 3. Post-Release Tasks
- [ ] Merge release branch to `main`
- [ ] Merge release branch to `develop`
- [ ] Create GitHub release with notes
- [ ] Update project documentation
- [ ] Notify stakeholders
- [ ] Monitor for issues

## 📊 Version Types

### Patch Release (0.1.0 → 0.1.1)
**Use for**: Bug fixes, documentation updates, minor improvements
- [ ] Only bug fixes and non-breaking changes
- [ ] No new features
- [ ] No API changes
- [ ] Backward compatible

### Minor Release (0.1.0 → 0.2.0)
**Use for**: New features, enhancements, new modules
- [ ] New features added
- [ ] New modules implemented
- [ ] API additions (backward compatible)
- [ ] Performance improvements
- [ ] New dependencies added

### Major Release (0.1.0 → 1.0.0)
**Use for**: Breaking changes, major refactoring
- [ ] Breaking API changes
- [ ] Removed deprecated features
- [ ] Major architectural changes
- [ ] Incompatible dependency updates
- [ ] Migration guide provided

## 🔍 Quality Gates

### Code Quality Gate
- [ ] Test coverage ≥ 80%
- [ ] No clippy warnings
- [ ] No compiler warnings
- [ ] All public APIs documented
- [ ] Examples provided for new features

### Security Gate
- [ ] No high/critical vulnerabilities
- [ ] Security audit passes
- [ ] Dependencies are up to date
- [ ] No hardcoded secrets

### Performance Gate
- [ ] No performance regressions
- [ ] Benchmarks pass
- [ ] Memory usage is optimized
- [ ] Build time is acceptable

### Documentation Gate
- [ ] README is up to date
- [ ] CHANGELOG is complete
- [ ] API docs are generated
- [ ] Examples are tested

## 🚨 Emergency Release Process

### For Critical Security Issues
- [ ] Create hotfix branch from `main`
- [ ] Implement minimal fix
- [ ] Run essential tests only
- [ ] Fast-track review process
- [ ] Deploy immediately
- [ ] Follow up with full release

### For Critical Bugs
- [ ] Assess impact and urgency
- [ ] Create hotfix branch
- [ ] Implement fix with tests
- [ ] Expedited review
- [ ] Deploy to production
- [ ] Document in CHANGELOG

## 📝 Release Notes Template

```markdown
## [Version] - YYYY-MM-DD

### Added
- New features and enhancements

### Changed
- Changes to existing functionality

### Fixed
- Bug fixes

### Removed
- Removed features (if any)

### Security
- Security improvements

### Performance
- Performance improvements

### Documentation
- Documentation updates

### Dependencies
- Dependency updates

### Breaking Changes
- Breaking changes (if any)

### Migration Guide
- Migration instructions (if needed)
```

## 🔧 Automated Checks

### CI/CD Pipeline
- [ ] All CI jobs pass
- [ ] Build succeeds on all platforms
- [ ] Tests pass on all Rust versions
- [ ] Documentation builds successfully
- [ ] Security scan passes

### Release Script Validation
- [ ] Release script runs without errors
- [ ] Version bump is correct
- [ ] Git tag is created
- [ ] Changelog is updated
- [ ] All files are committed

## 📞 Communication

### Internal Communication
- [ ] Notify team of release
- [ ] Update project status
- [ ] Document lessons learned

### External Communication
- [ ] Update project website
- [ ] Post release announcement
- [ ] Update social media
- [ ] Notify users of breaking changes

## 🎯 Success Criteria

A successful release should meet all of the following criteria:

- [ ] All tests pass
- [ ] No security vulnerabilities
- [ ] Documentation is complete
- [ ] Performance is maintained or improved
- [ ] Breaking changes are properly communicated
- [ ] Release is published successfully
- [ ] Users can upgrade without issues

## 📚 Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 🆘 Troubleshooting

### Common Issues
- **Build failures**: Check dependencies and Rust version
- **Test failures**: Review test environment and dependencies
- **Publish failures**: Verify crates.io credentials and permissions
- **Documentation issues**: Check markdown syntax and links

### Getting Help
- Check project documentation
- Review previous release notes
- Ask in project discussions
- Contact maintainers

---

**Remember**: Quality over speed. It's better to delay a release than to ship broken code.
