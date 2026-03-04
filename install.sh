#!/bin/bash
# PJMAI Install Script
# One-line install: curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash
#
# Options:
#   --no-shell    Skip shell integration (don't modify rc files)
#   --no-completions  Skip shell completion installation
#   --prefix DIR  Install to DIR instead of ~/.local/bin
#   --help        Show this help message

set -e

# Default configuration
REPO_URL="https://github.com/wrightmikea/pjmai"
INSTALL_PREFIX="${HOME}/.local/bin"
INSTALL_SHELL=true
INSTALL_COMPLETIONS=true
LOCAL_DIR=""
TEMP_DIR=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print functions
info() {
    echo -e "${BLUE}==>${NC} $1"
}

success() {
    echo -e "${GREEN}==>${NC} $1"
}

warn() {
    echo -e "${YELLOW}Warning:${NC} $1"
}

error() {
    echo -e "${RED}Error:${NC} $1" >&2
    exit 1
}

# Show help
show_help() {
    cat << EOF
PJMAI Install Script

Usage: install.sh [OPTIONS]

Options:
    --no-shell          Skip shell integration (don't modify rc files)
    --no-completions    Skip shell completion installation
    --prefix DIR        Install to DIR instead of ~/.local/bin
    --local DIR         Build from local directory instead of cloning from GitHub
    --help              Show this help message

Examples:
    # Standard installation
    curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash

    # Install to custom location
    curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash -s -- --prefix /usr/local/bin

    # Install without modifying shell rc files
    curl -fsSL https://raw.githubusercontent.com/wrightmikea/pjmai/main/install.sh | bash -s -- --no-shell

    # Install from local directory (for development)
    ./install.sh --local .
EOF
    exit 0
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --no-shell)
                INSTALL_SHELL=false
                shift
                ;;
            --no-completions)
                INSTALL_COMPLETIONS=false
                shift
                ;;
            --prefix)
                INSTALL_PREFIX="$2"
                shift 2
                ;;
            --local)
                LOCAL_DIR="$2"
                shift 2
                ;;
            --help|-h)
                show_help
                ;;
            *)
                error "Unknown option: $1. Use --help for usage."
                ;;
        esac
    done
}

# Cleanup function
cleanup() {
    if [[ -n "$TEMP_DIR" && -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
}

trap cleanup EXIT

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)
            OS="linux"
            ;;
        Darwin)
            OS="darwin"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            OS="windows"
            ;;
        *)
            error "Unsupported operating system: $OS"
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        arm64|aarch64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac

    info "Detected platform: ${OS}-${ARCH}"
}

# Detect user's shell
detect_shell() {
    if [[ -n "$ZSH_VERSION" ]] || [[ "$SHELL" == *"zsh"* ]]; then
        DETECTED_SHELL="zsh"
        RC_FILE="${HOME}/.zshrc"
    elif [[ -n "$BASH_VERSION" ]] || [[ "$SHELL" == *"bash"* ]]; then
        DETECTED_SHELL="bash"
        RC_FILE="${HOME}/.bashrc"
        # On macOS, bash uses .bash_profile for login shells
        if [[ "$OS" == "darwin" ]] && [[ -f "${HOME}/.bash_profile" ]]; then
            RC_FILE="${HOME}/.bash_profile"
        fi
    elif [[ "$SHELL" == *"fish"* ]]; then
        DETECTED_SHELL="fish"
        RC_FILE="${HOME}/.config/fish/config.fish"
    else
        DETECTED_SHELL="unknown"
        RC_FILE=""
    fi

    info "Detected shell: ${DETECTED_SHELL}"
}

# Check for required dependencies
check_dependencies() {
    local missing=()

    if ! command -v git &> /dev/null; then
        missing+=("git")
    fi

    if ! command -v cargo &> /dev/null; then
        missing+=("cargo (Rust)")
    fi

    if [[ ${#missing[@]} -gt 0 ]]; then
        error "Missing required dependencies: ${missing[*]}

Please install them first:
  - git: https://git-scm.com/
  - Rust/Cargo: https://rustup.rs/

On macOS with Homebrew:
  brew install git
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

On Ubuntu/Debian:
  sudo apt install git
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi

    success "All dependencies found"
}

# Clone and build from source
build_from_source() {
    local build_dir

    if [[ -n "$LOCAL_DIR" ]]; then
        # Build from local directory
        if [[ ! -d "$LOCAL_DIR" ]]; then
            error "Local directory not found: $LOCAL_DIR"
        fi

        build_dir="$(cd "$LOCAL_DIR" && pwd)"
        info "Building from local directory: ${build_dir}"
    else
        # Clone from GitHub
        info "Cloning repository..."
        TEMP_DIR="$(mktemp -d)"

        if ! git clone --depth 1 "$REPO_URL" "$TEMP_DIR/pjmai" 2>&1 | sed 's/^/    /'; then
            error "Failed to clone repository from $REPO_URL
Make sure the repository exists and is accessible."
        fi

        build_dir="$TEMP_DIR/pjmai"
    fi

    info "Building pjmai (this may take a moment)..."
    cd "$build_dir"
    cargo build --release 2>&1 | grep -E '^(   Compiling|    Finished)' | sed 's/^/    /' || true

    if [[ ! -f "target/release/pjmai" ]]; then
        error "Build failed: binary not found at target/release/pjmai"
    fi

    BINARY_PATH="${build_dir}/target/release/pjmai"
    SOURCE_SCRIPT_PATH="${build_dir}/source-pjm.sh"

    success "Build completed"
}

# Install the binary
install_binary() {
    info "Installing pjmai to ${INSTALL_PREFIX}..."

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_PREFIX"

    # Copy the binary
    cp "$BINARY_PATH" "$INSTALL_PREFIX/pjmai"
    chmod +x "$INSTALL_PREFIX/pjmai"

    # Verify installation
    if [[ -x "$INSTALL_PREFIX/pjmai" ]]; then
        success "Binary installed to ${INSTALL_PREFIX}/pjmai"
    else
        error "Failed to install binary"
    fi
}

# Install source-pjm.sh to a standard location
install_source_script() {
    local source_dir="${HOME}/.pjmai"

    info "Installing source-pjm.sh to ${source_dir}..."

    mkdir -p "$source_dir"
    cp "$SOURCE_SCRIPT_PATH" "$source_dir/source-pjm.sh"
    chmod +x "$source_dir/source-pjm.sh"

    SOURCE_SCRIPT_INSTALLED="${source_dir}/source-pjm.sh"
    success "Shell integration script installed"
}

# Add to PATH if needed
ensure_path() {
    # Check if INSTALL_PREFIX is already in PATH
    if [[ ":$PATH:" != *":$INSTALL_PREFIX:"* ]]; then
        info "Adding ${INSTALL_PREFIX} to PATH..."

        if [[ -n "$RC_FILE" ]]; then
            local path_line="export PATH=\"${INSTALL_PREFIX}:\$PATH\""

            # Check if already added
            if ! grep -q "$INSTALL_PREFIX" "$RC_FILE" 2>/dev/null; then
                echo "" >> "$RC_FILE"
                echo "# Added by pjmai installer" >> "$RC_FILE"
                echo "$path_line" >> "$RC_FILE"
                success "Added to PATH in ${RC_FILE}"
            else
                info "PATH already configured in ${RC_FILE}"
            fi
        else
            warn "Could not detect shell rc file. Please add ${INSTALL_PREFIX} to your PATH manually."
        fi
    else
        info "PATH already includes ${INSTALL_PREFIX}"
    fi
}

# Configure shell integration
configure_shell() {
    if [[ "$INSTALL_SHELL" != "true" ]]; then
        info "Skipping shell integration (--no-shell specified)"
        return
    fi

    info "Configuring shell integration..."

    local source_line="source \"${SOURCE_SCRIPT_INSTALLED}\""
    local marker="# PJMAI project management"

    if [[ -n "$RC_FILE" ]]; then
        # Check if already added
        if grep -q "source-pjm.sh" "$RC_FILE" 2>/dev/null; then
            info "Shell integration already configured in ${RC_FILE}"
        else
            echo "" >> "$RC_FILE"
            echo "$marker" >> "$RC_FILE"
            echo "$source_line" >> "$RC_FILE"
            success "Shell integration added to ${RC_FILE}"
        fi
    elif [[ "$DETECTED_SHELL" == "fish" ]]; then
        # Fish uses a different syntax
        warn "Fish shell detected. Please add manually to ${RC_FILE}:
  source ${SOURCE_SCRIPT_INSTALLED}"
    else
        warn "Could not detect shell rc file. Please add to your shell config:
  $source_line"
    fi
}

# Install shell completions
install_completions() {
    if [[ "$INSTALL_COMPLETIONS" != "true" ]]; then
        info "Skipping completions (--no-completions specified)"
        return
    fi

    # Need pjmai in PATH to generate completions
    export PATH="${INSTALL_PREFIX}:$PATH"

    if ! command -v pjmai &> /dev/null; then
        warn "pjmai not found in PATH, skipping completions"
        return
    fi

    info "Installing shell completions for ${DETECTED_SHELL}..."

    case "$DETECTED_SHELL" in
        zsh)
            local comp_dir="${HOME}/.zsh/completions"
            mkdir -p "$comp_dir"
            pjmai completions zsh > "$comp_dir/_pjmai"
            success "Zsh completions installed to ${comp_dir}/_pjmai"

            # Ensure fpath includes the completions directory
            if ! grep -q ".zsh/completions" "$RC_FILE" 2>/dev/null; then
                # Add fpath before compinit
                local fpath_line='fpath=(~/.zsh/completions $fpath)'
                echo "$fpath_line" >> "$RC_FILE"
                info "Added completions directory to fpath"
            fi
            ;;
        bash)
            local comp_dir="${HOME}/.local/share/bash-completion/completions"
            mkdir -p "$comp_dir"
            pjmai completions bash > "$comp_dir/pjmai"
            success "Bash completions installed to ${comp_dir}/pjmai"
            ;;
        fish)
            local comp_dir="${HOME}/.config/fish/completions"
            mkdir -p "$comp_dir"
            pjmai completions fish > "$comp_dir/pjmai.fish"
            success "Fish completions installed to ${comp_dir}/pjmai.fish"
            ;;
        *)
            warn "Unknown shell, skipping completions. Generate manually with:
  pjmai completions <shell>"
            ;;
    esac
}

# Verify installation
verify_installation() {
    info "Verifying installation..."

    export PATH="${INSTALL_PREFIX}:$PATH"

    if pjmai --version &> /dev/null; then
        local version
        version=$(pjmai --version 2>&1)
        success "Installation verified: ${version}"
    else
        error "Installation verification failed. pjmai --version did not succeed."
    fi
}

# Print final instructions
print_instructions() {
    echo ""
    echo "============================================"
    success "PJMAI installation complete!"
    echo "============================================"
    echo ""
    echo "To start using pjmai, reload your shell:"
    echo ""
    case "$DETECTED_SHELL" in
        zsh)
            echo "    source ~/.zshrc"
            ;;
        bash)
            echo "    source ${RC_FILE}"
            ;;
        fish)
            echo "    source ~/.config/fish/config.fish"
            ;;
        *)
            echo "    Restart your terminal or source your shell config"
            ;;
    esac
    echo ""
    echo "Quick start:"
    echo "    adpj myproject -f ~/path/to/project   # Add a project"
    echo "    lspj                                   # List projects"
    echo "    chpj myproject                         # Switch to project"
    echo "    hlpj                                   # Show all aliases"
    echo ""
    echo "For more information:"
    echo "    pjmai --help"
    echo "    ${REPO_URL}"
    echo ""
}

# Main installation flow
main() {
    echo ""
    echo "============================================"
    info "PJMAI Installer"
    echo "============================================"
    echo ""

    parse_args "$@"
    detect_platform
    detect_shell
    check_dependencies
    build_from_source
    install_binary
    install_source_script
    ensure_path
    configure_shell
    install_completions
    verify_installation
    print_instructions
}

main "$@"
