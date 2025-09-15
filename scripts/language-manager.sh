#!/bin/bash

# Language Manager Script for yimi-rutool
# This script helps manage bilingual documentation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONFIG_FILE="$PROJECT_ROOT/docs/language-config.json"

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if jq is installed
check_jq() {
    if ! command -v jq >/dev/null 2>&1; then
        print_error "jq is not installed. Please install jq to use this script."
        exit 1
    fi
}

# Function to get language configuration
get_language_config() {
    local lang="$1"
    local key="$2"
    jq -r ".languages.${lang}.${key}" "$CONFIG_FILE"
}

# Function to get all supported languages
get_supported_languages() {
    jq -r '.supported_languages[]' "$CONFIG_FILE"
}

# Function to generate language switcher HTML
generate_language_switcher() {
    local current_lang="$1"
    local current_file="$2"
    
    echo '<div align="center">'
    echo '  <h3>🌍 Language / 语言</h3>'
    echo '  <p>'
    
    local first=true
    for lang in $(get_supported_languages); do
        local lang_name=$(get_language_config "$lang" "name")
        local file_name=$(get_language_config "$lang" "$current_file")
        
        if [[ "$first" == "true" ]]; then
            first=false
        else
            echo -n " • "
        fi
        
        if [[ "$lang" == "$current_lang" ]]; then
            echo -n "<strong>$lang_name</strong>"
        else
            echo -n "<a href=\"$file_name\">$lang_name</a>"
        fi
    done
    
    echo
    echo '  </p>'
    echo '</div>'
}

# Function to update language switcher in a file
update_language_switcher() {
    local file_path="$1"
    local current_lang="$2"
    local file_type="$3"
    
    if [[ ! -f "$file_path" ]]; then
        print_warning "File $file_path does not exist, skipping..."
        return
    fi
    
    print_info "Updating language switcher in $file_path"
    
    # Create temporary file
    local temp_file=$(mktemp)
    
    # Generate new language switcher
    local switcher=$(generate_language_switcher "$current_lang" "$file_type")
    
    # Process the file
    local in_switcher=false
    while IFS= read -r line; do
        if [[ "$line" == *"🌍 Language / 语言"* ]]; then
            in_switcher=true
            echo "$switcher"
            continue
        fi
        
        if [[ "$in_switcher" == "true" && "$line" == *"</div>"* ]]; then
            in_switcher=false
            continue
        fi
        
        if [[ "$in_switcher" == "false" ]]; then
            echo "$line"
        fi
    done < "$file_path" > "$temp_file"
    
    # Replace original file
    mv "$temp_file" "$file_path"
    
    print_success "Updated language switcher in $file_path"
}

# Function to sync content between language versions
sync_content() {
    local source_lang="$1"
    local target_lang="$2"
    local file_type="$3"
    
    local source_file=$(get_language_config "$source_lang" "$file_type")
    local target_file=$(get_language_config "$target_lang" "$file_type")
    
    if [[ ! -f "$PROJECT_ROOT/$source_file" ]]; then
        print_error "Source file $source_file does not exist"
        return 1
    fi
    
    print_info "Syncing $file_type from $source_lang to $target_lang"
    
    # Copy content (excluding language switcher)
    local temp_file=$(mktemp)
    local in_switcher=false
    
    while IFS= read -r line; do
        if [[ "$line" == *"🌍 Language / 语言"* ]]; then
            in_switcher=true
            continue
        fi
        
        if [[ "$in_switcher" == "true" && "$line" == *"</div>"* ]]; then
            in_switcher=false
            continue
        fi
        
        if [[ "$in_switcher" == "false" ]]; then
            echo "$line"
        fi
    done < "$PROJECT_ROOT/$source_file" > "$temp_file"
    
    # Add target language switcher
    local switcher=$(generate_language_switcher "$target_lang" "$file_type")
    {
        echo "$switcher"
        echo
        cat "$temp_file"
    } > "$PROJECT_ROOT/$target_file"
    
    rm "$temp_file"
    
    print_success "Synced $file_type to $target_file"
}

# Function to show help
show_help() {
    cat << EOF
yimi-rutool Language Manager

Usage: $0 [OPTIONS] COMMAND

Commands:
    update-switchers    Update language switchers in all files
    sync <source> <target> <type>  Sync content between language versions
    list-languages      List all supported languages
    check               Check if all language files exist

Options:
    --help              Show this help message

Examples:
    $0 update-switchers                    # Update all language switchers
    $0 sync zh en readme                   # Sync README from Chinese to English
    $0 sync en zh changelog                # Sync CHANGELOG from English to Chinese
    $0 list-languages                      # List supported languages
    $0 check                               # Check file existence

EOF
}

# Function to list supported languages
list_languages() {
    print_info "Supported languages:"
    for lang in $(get_supported_languages); do
        local lang_name=$(get_language_config "$lang" "name")
        echo "  - $lang ($lang_name)"
    done
}

# Function to check if all language files exist
check_files() {
    print_info "Checking language files..."
    
    local missing_files=()
    
    for lang in $(get_supported_languages); do
        for file_type in readme changelog contributing branch_strategy release_checklist version_management; do
            local file_name=$(get_language_config "$lang" "$file_type")
            local file_path="$PROJECT_ROOT/$file_name"
            
            if [[ ! -f "$file_path" ]]; then
                missing_files+=("$file_path")
            fi
        done
    done
    
    if [[ ${#missing_files[@]} -eq 0 ]]; then
        print_success "All language files exist"
    else
        print_warning "Missing files:"
        for file in "${missing_files[@]}"; do
            echo "  - $file"
        done
    fi
}

# Main function
main() {
    local command=""
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            update-switchers|sync|list-languages|check)
                command="$1"
                shift
                break
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    if [[ -z "$command" ]]; then
        print_error "No command specified"
        show_help
        exit 1
    fi
    
    # Check prerequisites
    check_jq
    
    if [[ ! -f "$CONFIG_FILE" ]]; then
        print_error "Language configuration file not found: $CONFIG_FILE"
        exit 1
    fi
    
    print_info "Starting language management..."
    
    case "$command" in
        update-switchers)
            for lang in $(get_supported_languages); do
                for file_type in readme changelog; do
                    local file_name=$(get_language_config "$lang" "$file_type")
                    local file_path="$PROJECT_ROOT/$file_name"
                    update_language_switcher "$file_path" "$lang" "$file_type"
                done
            done
            ;;
        sync)
            if [[ $# -lt 3 ]]; then
                print_error "sync command requires source language, target language, and file type"
                exit 1
            fi
            sync_content "$1" "$2" "$3"
            ;;
        list-languages)
            list_languages
            ;;
        check)
            check_files
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
    
    print_success "Language management completed!"
}

# Run main function with all arguments
main "$@"
