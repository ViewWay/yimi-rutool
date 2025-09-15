# Bilingual Documentation Management

This document explains how to manage the bilingual (Chinese/English) documentation system for yimi-rutool.

## 📁 File Structure

```
yimi-rutool/
├── README.md              # Chinese README (default)
├── README-EN.md           # English README
├── CHANGELOG.md           # Chinese CHANGELOG (default)
├── CHANGELOG-EN.md        # English CHANGELOG
├── CONTRIBUTING.md        # Chinese Contributing Guide
├── CONTRIBUTING-EN.md     # English Contributing Guide (to be created)
├── BRANCH_STRATEGY.md     # Chinese Branch Strategy
├── BRANCH_STRATEGY-EN.md  # English Branch Strategy (to be created)
├── RELEASE_CHECKLIST.md   # Chinese Release Checklist
├── RELEASE_CHECKLIST-EN.md # English Release Checklist (to be created)
├── VERSION_MANAGEMENT.md  # Chinese Version Management
├── VERSION_MANAGEMENT-EN.md # English Version Management (to be created)
├── docs/
│   ├── language-config.json    # Language configuration
│   └── BILINGUAL_DOCUMENTATION.md # This file
└── scripts/
    └── language-manager.sh     # Language management script
```

## 🌍 Language Configuration

The language configuration is stored in `docs/language-config.json`:

```json
{
  "languages": {
    "zh": {
      "name": "中文",
      "code": "zh",
      "readme": "README.md",
      "changelog": "CHANGELOG.md",
      "contributing": "CONTRIBUTING.md",
      "branch_strategy": "BRANCH_STRATEGY.md",
      "release_checklist": "RELEASE_CHECKLIST.md",
      "version_management": "VERSION_MANAGEMENT.md"
    },
    "en": {
      "name": "English",
      "code": "en",
      "readme": "README-EN.md",
      "changelog": "CHANGELOG-EN.md",
      "contributing": "CONTRIBUTING-EN.md",
      "branch_strategy": "BRANCH_STRATEGY-EN.md",
      "release_checklist": "RELEASE_CHECKLIST-EN.md",
      "version_management": "VERSION_MANAGEMENT-EN.md"
    }
  },
  "default_language": "zh",
  "supported_languages": ["zh", "en"]
}
```

## 🔧 Language Management Script

The `scripts/language-manager.sh` script provides automated management of bilingual documentation.

### Available Commands

#### 1. Update Language Switchers
```bash
./scripts/language-manager.sh update-switchers
```
Updates the language switcher in all documentation files.

#### 2. Sync Content Between Languages
```bash
# Sync README from Chinese to English
./scripts/language-manager.sh sync zh en readme

# Sync CHANGELOG from English to Chinese
./scripts/language-manager.sh sync en zh changelog
```

#### 3. List Supported Languages
```bash
./scripts/language-manager.sh list-languages
```

#### 4. Check File Existence
```bash
./scripts/language-manager.sh check
```

## 📝 Language Switcher Format

Each documentation file includes a language switcher at the top:

```html
<div align="center">
  <h3>🌍 Language / 语言</h3>
  <p>
    <a href="README.md">中文</a> •
    <a href="README-EN.md">English</a>
  </p>
</div>
```

## 🔄 Workflow for Adding New Content

### 1. Add Content to Default Language (Chinese)
- Edit the Chinese version of the file (e.g., `README.md`)
- Add new content in Chinese

### 2. Sync to English Version
```bash
./scripts/language-manager.sh sync zh en readme
```

### 3. Translate Content
- Edit the English version (e.g., `README-EN.md`)
- Translate the new content to English
- Keep the language switcher intact

### 4. Update Language Switchers
```bash
./scripts/language-manager.sh update-switchers
```

## 📋 Best Practices

### Content Management
1. **Default Language**: Chinese is the default language for new content
2. **Consistency**: Keep both language versions in sync
3. **Translation Quality**: Ensure accurate and natural translations
4. **Cultural Adaptation**: Adapt examples and references for different audiences

### File Naming
- Chinese files: `FILENAME.md`
- English files: `FILENAME-EN.md`
- Configuration files: `docs/language-config.json`

### Language Switcher
- Always include at the top of each documentation file
- Use consistent format across all files
- Update when adding new languages

### Version Control
- Commit both language versions together
- Use conventional commit messages
- Include language information in commit messages

## 🚀 Adding New Languages

To add a new language (e.g., Japanese):

### 1. Update Configuration
Edit `docs/language-config.json`:
```json
{
  "languages": {
    "zh": { ... },
    "en": { ... },
    "ja": {
      "name": "日本語",
      "code": "ja",
      "readme": "README-JA.md",
      "changelog": "CHANGELOG-JA.md",
      "contributing": "CONTRIBUTING-JA.md",
      "branch_strategy": "BRANCH_STRATEGY-JA.md",
      "release_checklist": "RELEASE_CHECKLIST-JA.md",
      "version_management": "VERSION_MANAGEMENT-JA.md"
    }
  },
  "supported_languages": ["zh", "en", "ja"]
}
```

### 2. Create Language Files
```bash
# Copy existing files and translate
cp README.md README-JA.md
cp CHANGELOG.md CHANGELOG-JA.md
# ... etc
```

### 3. Update Language Switchers
```bash
./scripts/language-manager.sh update-switchers
```

## 🔍 Quality Assurance

### Automated Checks
- Use the language manager script to check file existence
- Verify language switchers are consistent
- Ensure all language versions are up to date

### Manual Review
- Review translations for accuracy
- Check cultural appropriateness
- Verify technical terms are consistent

### Testing
- Test language switcher links
- Verify all files are accessible
- Check formatting consistency

## 📞 Support

For questions about bilingual documentation:
- Check this guide first
- Review the language manager script
- Ask in project discussions
- Contact maintainers

---

This bilingual documentation system ensures that yimi-rutool is accessible to both Chinese and English-speaking developers, promoting international collaboration and adoption.
