# Branch Strategy for yimi-rutool

This document outlines the branching strategy and workflow for the yimi-rutool project.

## 🌿 Branch Types

### Main Branches

#### `main`
- **Purpose**: Production-ready code
- **Protection**: Protected branch with required reviews
- **Deployment**: Automatically deployed to production
- **Merging**: Only through pull requests from other branches

#### `develop`
- **Purpose**: Integration branch for features
- **Protection**: Protected branch with required reviews
- **Merging**: Feature branches merge here first
- **Testing**: Continuous integration runs on this branch

### Supporting Branches

#### Feature Branches
- **Naming**: `feature/description` (e.g., `feature/bloom-filter`)
- **Source**: `develop` branch
- **Destination**: `develop` branch
- **Purpose**: New feature development
- **Lifecycle**: Deleted after merging

#### Bug Fix Branches
- **Naming**: `fix/description` (e.g., `fix/memory-leak`)
- **Source**: `main` branch (for hotfixes) or `develop` (for regular fixes)
- **Destination**: `main` and `develop` branches
- **Purpose**: Bug fixes
- **Lifecycle**: Deleted after merging

#### Release Branches
- **Naming**: `release/version` (e.g., `release/0.2.4`)
- **Source**: `develop` branch
- **Destination**: `main` and `develop` branches
- **Purpose**: Release preparation and final testing
- **Lifecycle**: Deleted after release

#### Hotfix Branches
- **Naming**: `hotfix/description` (e.g., `hotfix/security-patch`)
- **Source**: `main` branch
- **Destination**: `main` and `develop` branches
- **Purpose**: Critical production fixes
- **Lifecycle**: Deleted after merging

## 🔄 Workflow

### Feature Development

1. **Create Feature Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/new-feature
   ```

2. **Develop Feature**
   - Make commits following conventional commit format
   - Write tests for new functionality
   - Update documentation
   - Ensure all tests pass

3. **Submit Pull Request**
   - Create PR from `feature/new-feature` to `develop`
   - Include description of changes
   - Reference related issues
   - Request code review

4. **Code Review**
   - At least one approval required
   - Address review feedback
   - Ensure CI passes

5. **Merge and Cleanup**
   - Merge PR to `develop`
   - Delete feature branch
   - Update local branches

### Bug Fix Process

1. **Create Fix Branch**
   ```bash
   git checkout develop  # or main for hotfixes
   git pull origin develop
   git checkout -b fix/bug-description
   ```

2. **Implement Fix**
   - Write failing test first (if applicable)
   - Implement fix
   - Ensure all tests pass
   - Update documentation if needed

3. **Submit Pull Request**
   - Create PR to appropriate target branch
   - Include description of the bug and fix
   - Reference related issues

4. **Review and Merge**
   - Code review process
   - Merge to target branch
   - Delete fix branch

### Release Process

1. **Create Release Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b release/0.2.4
   ```

2. **Prepare Release**
   - Update version numbers
   - Update CHANGELOG.md
   - Final testing
   - Documentation updates

3. **Merge to Main**
   ```bash
   git checkout main
   git merge release/0.2.4
   git tag -a v0.2.4 -m "Release version 0.2.4"
   git push origin main --tags
   ```

4. **Merge Back to Develop**
   ```bash
   git checkout develop
   git merge release/0.2.4
   git push origin develop
   ```

5. **Cleanup**
   ```bash
   git branch -d release/0.2.4
   git push origin --delete release/0.2.4
   ```

## 📋 Branch Naming Conventions

### Format
```
<type>/<description>
```

### Types
- **feature/**: New features
- **fix/**: Bug fixes
- **hotfix/**: Critical production fixes
- **release/**: Release preparation
- **docs/**: Documentation updates
- **refactor/**: Code refactoring
- **perf/**: Performance improvements
- **chore/**: Maintenance tasks

### Examples
```bash
feature/bloom-filter
fix/memory-leak
hotfix/security-patch
release/0.2.4
docs/api-reference
refactor/cache-module
perf/hash-functions
chore/update-dependencies
```

## 🔒 Branch Protection Rules

### Main Branch Protection
- Require pull request reviews before merging
- Require status checks to pass before merging
- Require branches to be up to date before merging
- Restrict pushes that create files larger than 100MB
- Require linear history

### Develop Branch Protection
- Require pull request reviews before merging
- Require status checks to pass before merging
- Allow force pushes (for rebasing)
- Allow deletions

## 🚀 Continuous Integration

### Branch Triggers
- **Push to any branch**: Run basic checks (format, clippy, tests)
- **Pull request**: Run full CI pipeline
- **Push to main**: Run deployment pipeline
- **Tag creation**: Trigger release process

### Required Checks
- Code formatting (`cargo fmt --check`)
- Linting (`cargo clippy`)
- Unit tests (`cargo test`)
- Documentation tests (`cargo test --doc`)
- Security audit (`cargo audit`)

## 📝 Commit Message Guidelines

Follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types
- **feat**: New features
- **fix**: Bug fixes
- **docs**: Documentation changes
- **style**: Code style changes
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Test changes
- **chore**: Maintenance tasks

### Examples
```bash
feat(algorithms): add bloom filter implementation
fix(crypto): resolve RSA timing vulnerability
docs: update API documentation
style: format code with cargo fmt
refactor(cache): improve LRU cache performance
perf(hash): optimize hash function performance
test: add unit tests for text processing
chore: update dependencies
```

## 🔄 Branch Lifecycle

### Creation
- Create from appropriate source branch
- Use descriptive naming
- Include issue reference if applicable

### Development
- Make small, focused commits
- Write meaningful commit messages
- Keep branch up to date with source
- Run tests frequently

### Review
- Create pull request when ready
- Request appropriate reviewers
- Address feedback promptly
- Keep PR focused and small

### Merging
- Squash commits if appropriate
- Delete branch after merging
- Update local branches
- Clean up remote references

## 🛠️ Tools and Automation

### Git Hooks
- Pre-commit: Run formatting and basic checks
- Pre-push: Run tests and linting
- Commit-msg: Validate commit message format

### GitHub Actions
- Automated testing on all branches
- Automated deployment on main
- Automated release on tags
- Automated security scanning

### Branch Management Scripts
```bash
# Create feature branch
./scripts/create-feature.sh feature-name

# Create fix branch
./scripts/create-fix.sh bug-description

# Create release branch
./scripts/create-release.sh version

# Clean up merged branches
./scripts/cleanup-branches.sh
```

## 📚 Best Practices

### Branch Management
- Keep branches focused and small
- Delete branches after merging
- Use descriptive branch names
- Keep branches up to date
- Avoid long-lived feature branches

### Code Quality
- Write tests for new features
- Update documentation
- Follow coding standards
- Use meaningful commit messages
- Review code thoroughly

### Collaboration
- Communicate changes clearly
- Request appropriate reviewers
- Respond to feedback promptly
- Help others with their PRs
- Share knowledge and best practices

## 🚨 Emergency Procedures

### Hotfix Process
1. Create hotfix branch from main
2. Implement minimal fix
3. Test thoroughly
4. Create PR to main
5. Fast-track review process
6. Merge to main and develop
7. Create new release
8. Deploy immediately

### Rollback Process
1. Identify problematic commit
2. Create revert commit
3. Test revert thoroughly
4. Deploy revert
5. Investigate root cause
6. Plan proper fix

This branching strategy ensures code quality, collaboration efficiency, and reliable releases while maintaining a clean and organized repository structure.
