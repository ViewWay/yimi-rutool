#!/bin/bash

# yimi-rutool Release Script
# This script automates the release process for yimi-rutool

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

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check prerequisites
check_prerequisites() {
    print_info "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "cargo is not installed"
        exit 1
    fi
    
    if ! command_exists git; then
        print_error "git is not installed"
        exit 1
    fi
    
    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Not in a git repository"
        exit 1
    fi
    
    # Check if working directory is clean
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory is not clean. Please commit or stash changes."
        exit 1
    fi
    
    # Check if we're on main branch
    current_branch=$(git branch --show-current)
    if [[ "$current_branch" != "main" && "$current_branch" != "master" ]]; then
        print_error "Not on main/master branch. Current branch: $current_branch"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to run tests
run_tests() {
    print_info "Running tests..."
    
    cd "$PROJECT_ROOT"
    
    # Run unit tests
    print_info "Running unit tests..."
    if ! cargo test; then
        print_error "Unit tests failed"
        exit 1
    fi
    
    # Run documentation tests
    print_info "Running documentation tests..."
    if ! cargo test --doc; then
        print_error "Documentation tests failed"
        exit 1
    fi
    
    # Run clippy
    print_info "Running clippy..."
    if ! cargo clippy --all-targets --all-features -- -D warnings; then
        print_error "Clippy found issues"
        exit 1
    fi
    
    # Run cargo audit
    print_info "Running security audit..."
    if command_exists cargo-audit; then
        if ! cargo audit; then
            print_warning "Security audit found issues (check output above)"
        fi
    else
        print_warning "cargo-audit not installed, skipping security audit"
    fi
    
    print_success "All tests passed"
}

# Function to update version
update_version() {
    local version_type="$1"
    
    print_info "Updating version ($version_type)..."
    
    cd "$PROJECT_ROOT"
    
    # Update version in Cargo.toml
    if ! cargo release "$version_type" --no-publish --no-push --no-tag; then
        print_error "Failed to update version"
        exit 1
    fi
    
    print_success "Version updated successfully"
}

# Function to update changelog
update_changelog() {
    print_info "Updating changelog..."
    
    cd "$PROJECT_ROOT"
    
    # Get current version
    current_version=$(cargo metadata --format-version 1 | jq -r '.packages[0].version')
    
    # Update CHANGELOG.md (this would typically be done by cargo-release)
    print_info "Changelog will be updated by cargo-release"
    
    print_success "Changelog updated"
}

# Function to create release
create_release() {
    local version_type="$1"
    local dry_run="${2:-false}"
    
    print_info "Creating release ($version_type)..."
    
    cd "$PROJECT_ROOT"
    
    if [[ "$dry_run" == "true" ]]; then
        print_info "Running in dry-run mode..."
        cargo release "$version_type" --dry-run
    else
        # Full release process
        cargo release "$version_type"
    fi
    
    print_success "Release created successfully"
}

# Function to show help
show_help() {
    cat << EOF
yimi-rutool Release Script

Usage: $0 [OPTIONS] COMMAND

Commands:
    patch       Create a patch release (0.1.0 -> 0.1.1)
    minor       Create a minor release (0.1.0 -> 0.2.0)
    major       Create a major release (0.1.0 -> 1.0.0)
    test        Run all tests without releasing
    check       Check prerequisites only

Options:
    --dry-run   Show what would be done without actually doing it
    --help      Show this help message

Examples:
    $0 patch              # Create patch release
    $0 minor --dry-run    # Show what minor release would do
    $0 test               # Run tests only

EOF
}

# Main function
main() {
    local command=""
    local dry_run="false"
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            patch|minor|major|test|check)
                command="$1"
                shift
                ;;
            --dry-run)
                dry_run="true"
                shift
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
    
    print_info "Starting yimi-rutool release process..."
    print_info "Command: $command"
    print_info "Dry run: $dry_run"
    
    # Always check prerequisites
    check_prerequisites
    
    case "$command" in
        test)
            run_tests
            ;;
        check)
            print_success "Prerequisites check completed"
            ;;
        patch|minor|major)
            run_tests
            create_release "$command" "$dry_run"
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
    
    print_success "Release process completed successfully!"
}

# Run main function with all arguments
main "$@"
