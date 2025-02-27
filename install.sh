#!/bin/sh
# Smart Tree Installation Script

set -e

GITHUB_REPO="erik-balfe/smart-tree"
BINARY_NAME="smart-tree"
INSTALL_DIR="/usr/local/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print message with color
print_message() {
  printf "${2}${1}${NC}\n"
}

# Detect platform
detect_platform() {
  OS="$(uname -s)"
  ARCH="$(uname -m)"

  case "$OS" in
    Linux)
      PLATFORM="linux"
      ;;
    Darwin)
      PLATFORM="macos"
      ;;
    MINGW* | MSYS* | CYGWIN*)
      PLATFORM="windows"
      ;;
    *)
      print_message "Unsupported operating system: $OS" "$RED"
      exit 1
      ;;
  esac

  case "$ARCH" in
    x86_64 | amd64)
      ARCHITECTURE="amd64"
      ;;
    arm64 | aarch64)
      ARCHITECTURE="arm64"
      print_message "Warning: ARM64 is not officially supported yet, trying with amd64 version" "$YELLOW"
      ARCHITECTURE="amd64"
      ;;
    *)
      print_message "Unsupported architecture: $ARCH" "$RED"
      exit 1
      ;;
  esac

  if [ "$PLATFORM" = "windows" ]; then
    BINARY_URL="https://github.com/${GITHUB_REPO}/releases/latest/download/${BINARY_NAME}-${PLATFORM}-${ARCHITECTURE}.exe"
    BINARY_PATH="${BINARY_NAME}.exe"
  else
    BINARY_URL="https://github.com/${GITHUB_REPO}/releases/latest/download/${BINARY_NAME}-${PLATFORM}-${ARCHITECTURE}"
    BINARY_PATH="${BINARY_NAME}"
  fi
}

# Check if curl or wget is installed
check_downloader() {
  if command -v curl >/dev/null 2>&1; then
    DOWNLOADER="curl"
  elif command -v wget >/dev/null 2>&1; then
    DOWNLOADER="wget"
  else
    print_message "Error: neither curl nor wget is installed. Please install one of them and try again." "$RED"
    exit 1
  fi
}

# Download the binary
download_binary() {
  TMP_DIR="$(mktemp -d)"
  TMP_FILE="${TMP_DIR}/${BINARY_PATH}"

  print_message "Downloading ${BINARY_NAME} for ${PLATFORM}/${ARCHITECTURE}..." "$BLUE"
  
  if [ "$DOWNLOADER" = "curl" ]; then
    curl -sL "$BINARY_URL" -o "$TMP_FILE"
  else
    wget -q "$BINARY_URL" -O "$TMP_FILE"
  fi

  if [ ! -f "$TMP_FILE" ]; then
    print_message "Failed to download ${BINARY_NAME}" "$RED"
    exit 1
  fi
}

# Install the binary
install_binary() {
  chmod +x "$TMP_FILE"

  # Check if we need sudo
  if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_FILE" "${INSTALL_DIR}/${BINARY_PATH}"
  else
    print_message "Elevated permissions required to install to ${INSTALL_DIR}" "$YELLOW"
    sudo mv "$TMP_FILE" "${INSTALL_DIR}/${BINARY_PATH}"
  fi

  rm -rf "$TMP_DIR"
  
  if command -v "$BINARY_NAME" >/dev/null 2>&1; then
    print_message "${BINARY_NAME} installed successfully to ${INSTALL_DIR}/${BINARY_PATH}" "$GREEN"
    print_message "Run '${BINARY_NAME} --help' to get started" "$BLUE"
  else
    print_message "Installation successful, but ${BINARY_NAME} is not in your PATH" "$YELLOW"
    print_message "Installed to: ${INSTALL_DIR}/${BINARY_PATH}" "$BLUE"
  fi
}

main() {
  print_message "Smart Tree Installer" "$GREEN"
  
  check_downloader
  detect_platform
  download_binary
  install_binary
}

main