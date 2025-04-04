#!/bin/bash
set -euo pipefail

# Colors for output
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
RESET="\033[0m"

# Function to display usage instructions
usage() {
    echo -e "${BLUE}Smart Tree Release Tool${RESET}"
    echo
    echo "This script helps manage version and releases for the Smart Tree project"
    echo
    echo "Usage:"
    echo "  ./release.sh [command]"
    echo
    echo "Commands:"
    echo "  major      - Bump major version (X.0.0)"
    echo "  minor      - Bump minor version (0.X.0)"
    echo "  patch      - Bump patch version (0.0.X)"
    echo "  current    - Show current version"
    echo "  help       - Show this help message"
    echo
    echo "Examples:"
    echo "  ./release.sh minor    # Increases minor version (e.g., 0.2.0 -> 0.3.0)"
    echo "  ./release.sh patch    # Increases patch version (e.g., 0.2.0 -> 0.2.1)"
    echo
}

# Function to extract current version from Cargo.toml
get_current_version() {
    grep -m 1 '^version =' Cargo.toml | awk -F '"' '{print $2}' | tr -d '[:space:]'
}

# Function to update version in Cargo.toml
update_version_in_cargo_toml() {
    local new_version=$1
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    echo -e "${GREEN}Updated Cargo.toml to version $new_version${RESET}"
}

# Function to tag release in Jujutsu
tag_release() {
    local new_version=$1
    local tag_name="v$new_version"
    local message="Release $tag_name"
    
    # Make sure the file is tracked by jj
    jj file track release.sh || echo "release.sh already tracked"
    
    # Create a new change for the version update
    jj new -m "Bump version to $new_version"
    
    # Create tag (bookmark) in Jujutsu
    jj bookmark set "$tag_name" -r @ -B
    
    echo -e "${GREEN}Created bookmark/tag $tag_name${RESET}"
    
    # Optional: Push changes to remote
    read -p "Push changes to remote? (y/n): " push_choice
    if [[ $push_choice == "y" || $push_choice == "Y" ]]; then
        jj bookmark set master -r @ -B
        jj git push -b master
        echo -e "${GREEN}Changes pushed to remote${RESET}"
        echo
        echo -e "${YELLOW}Now go to GitHub and create a release from tag $tag_name${RESET}"
    else
        echo -e "${YELLOW}Remember to push changes with: jj bookmark set master -r @ -B && jj git push -b master${RESET}"
    fi
}

# Function to bump version
bump_version() {
    local component=$1
    local current_version=$(get_current_version)
    local major minor patch
    
    # Parse version components
    IFS='.' read -r major minor patch <<< "$current_version"
    
    # Update appropriate component
    case $component in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        *)
            echo -e "${RED}Invalid version component: $component${RESET}"
            exit 1
            ;;
    esac
    
    local new_version="$major.$minor.$patch"
    
    echo -e "${BLUE}Current version: ${RESET}$current_version"
    echo -e "${BLUE}New version:     ${RESET}$new_version"
    
    # Confirm before proceeding
    read -p "Proceed with version update? (y/n): " choice
    if [[ $choice != "y" && $choice != "Y" ]]; then
        echo -e "${YELLOW}Version update cancelled${RESET}"
        exit 0
    fi
    
    # Update version in files
    update_version_in_cargo_toml "$new_version"
    
    # Tag the release
    tag_release "$new_version"
}

# Main script execution
command=${1:-help}

case $command in
    major|minor|patch)
        bump_version "$command"
        ;;
    current)
        current_version=$(get_current_version)
        echo -e "${BLUE}Current version: ${RESET}$current_version"
        ;;
    help)
        usage
        ;;
    *)
        echo -e "${RED}Unknown command: $command${RESET}"
        usage
        exit 1
        ;;
esac

exit 0