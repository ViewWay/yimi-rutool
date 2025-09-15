# Branch Strategy for yimi-rutool

This document outlines the branching strategy and workflow for the yimi-rutool project.

## 🌿 Branch Types

### Main Branches

#### `main`
- **Purpose**: Production-ready code
- **Protection**: Protected branch with required reviews
- **Deployment**: Automatically deployed to production
- **Merging**: Only through pull requests from `develop` or `release/*` branches

#### `develop`
- **Purpose**: Integration branch for features
- **Protection**: Protected branch with required reviews
- **Merging**: Feature branches merge here first
- **Deployment**: Automatically deployed to staging

### Supporting Branches

#### Feature Branches
- **Naming**: `feature/<feature-name>`
- **Purpose**: New features or enhancements
- **Lifecycle**: Created from `develop`, merged back to `develop`
- **Examples**:
  - `feature/bloom-filter`
  - `feature/text-processing`
  - `feature/jwt-authentication`

#### Bug Fix Branches
- **Naming**: `fix/<bug-description>`
- **Purpose**: Bug fixes
- **Lifecycle**: Created from `develop`, merged back to `develop`
- **Examples**:
  - `fix/memory-leak`
  - `fix/rsa-vulnerability`
  - `fix/http-timeout`

#### Hotfix Branches
- **Naming**: `hotfix/<issue-description>`
- **Purpose**: Critical bug fixes for production
- **Lifecycle**: Created from `main`, merged to both `main` and `develop`
- **Examples**:
  - `hotfix/security-patch`
  - `hotfix/critical-bug`

#### Release Branches
- **Naming**: `release/<version>`
- **Purpose**: Release preparation
- **Lifecycle**: Created from `develop`, merged to `main` and `develop`
- **Examples**:
  - `release/v0.3.0`
  - `release/v0.2.5`

#### Documentation Branches
- **Naming**: `docs/<documentation-type>`
- **Purpose**: Documentation updates
- **Lifecycle**: Created from `develop`, merged back to `develop`
- **Examples**:
  - `docs/api-reference`
  - `docs/examples`
  - `docs/contributing`

#### Refactoring Branches
- **Naming**: `refactor/<component-name>`
- **Purpose**: Code refactoring without new features
- **Lifecycle**: Created from `develop`, merged back to `develop`
- **Examples**:
  - `refactor/cache-module`
  - `refactor/error-handling`

#### Performance Branches
- **Naming**: `perf/<optimization-area>`
- **Purpose**: Performance improvements
- **Lifecycle**: Created from `develop`, merged back to `develop`
- **Examples**:
  - `perf/hash-functions`
  - `perf/memory-usage`

#### Chore Branches
- **Naming**: `chore/<task-description>`
- **Purpose**: Maintenance tasks
- **Lifecycle**: Created from `develop`, merged back to `develop`
- **Examples**:
  - `chore/update-dependencies`
  - `chore/ci-improvements`

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

3. **Create Pull Request**
   - Target: `develop` branch
   - Include description of changes
   - Reference related issues
   - Request code review

4. **Code Review**
   - At least one approval required
   - Address review feedback
   - Ensure CI passes

5. **Merge**
   - Use "Squash and merge" for clean history
   - Delete feature branch after merge

### Bug Fix Workflow

1. **Create Fix Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b fix/bug-description
   ```

2. **Fix Bug**
   - Write test that reproduces the bug
   - Implement fix
   - Ensure test passes
   - Update documentation if needed

3. **Create Pull Request**
   - Target: `develop` branch
   - Include bug description and fix details
   - Reference issue number

4. **Review and Merge**
   - Same process as feature branches

### Hotfix Workflow

1. **Create Hotfix Branch**
   ```bash
   git checkout main
   git pull origin main
   git checkout -b hotfix/critical-issue
   ```

2. **Fix Critical Issue**
   - Implement minimal fix
   - Write tests
   - Update version if needed

3. **Create Pull Request**
   - Target: `main` branch
   - Mark as urgent
   - Include impact assessment

4. **Merge to Main**
   - Fast-track review process
   - Merge to `main`

5. **Merge to Develop**
   - Create PR from `main` to `develop`
   - Ensure fix is included in next release

### Release Workflow

1. **Create Release Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b release/v0.3.0
   ```

2. **Prepare Release**
   - Update version numbers
   - Update CHANGELOG.md
   - Run full test suite
   - Update documentation

3. **Create Pull Request**
   - Target: `main` branch
   - Include release notes
   - Request final review

4. **Merge to Main**
   - Merge release branch to `main`
   - Create and push tag
   - Trigger release workflow

5. **Merge to Develop**
   - Merge release branch to `develop`
   - Clean up release branch

## 🛡️ Branch Protection Rules

### Main Branch
- Require pull request reviews (1 reviewer minimum)
- Require status checks to pass before merging
- Require branches to be up to date before merging
- Restrict pushes to main branch
- Require linear history

### Develop Branch
- Require pull request reviews (1 reviewer minimum)
- Require status checks to pass before merging
- Allow force pushes (for rebasing)
- Require linear history

## 📋 Branch Naming Conventions

### Format
```
<type>/<description>
```

### Types
- `feature/`: New features
- `fix/`: Bug fixes
- `hotfix/`: Critical bug fixes
- `release/`: Release preparation
- `docs/`: Documentation updates
- `refactor/`: Code refactoring
- `perf/`: Performance improvements
- `chore/`: Maintenance tasks

### Description Guidelines
- Use kebab-case (lowercase with hyphens)
- Be descriptive but concise
- Include issue number if applicable
- Examples:
  - `feature/bloom-filter-implementation`
  - `fix/memory-leak-in-cache`
  - `docs/api-documentation-update`

## 🧹 Branch Cleanup

### Automatic Cleanup
- Feature branches are automatically deleted after merge
- Release branches are deleted after release completion
- Hotfix branches are deleted after merge to both main and develop

### Manual Cleanup
- Regularly clean up stale branches
- Remove branches that are no longer needed
- Use `git branch --merged` to find merged branches

## 🔍 Branch Status Tracking

### Branch Health
- Monitor branch age
- Track merge frequency
- Identify long-running branches
- Monitor CI/CD status

### Metrics
- Average time to merge
- Number of open pull requests
- Branch divergence from main
- Test coverage per branch

## 🚀 Best Practices

### Do's
- Create branches from the correct base branch
- Use descriptive branch names
- Keep branches focused on single purpose
- Regularly sync with base branch
- Delete branches after merge
- Use conventional commit messages

### Don'ts
- Don't create branches from outdated base branches
- Don't use generic branch names
- Don't mix unrelated changes in one branch
- Don't let branches become stale
- Don't force push to shared branches
- Don't merge without proper review

## 📞 Support

For questions about branching strategy:
- Create an issue in the repository
- Ask in project discussions
- Contact maintainers

---

This branching strategy ensures a clean, organized, and efficient development workflow for the yimi-rutool project.
