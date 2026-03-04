#!/bin/bash
# PJMAI Uninstall Script
# Removes pjmai binary, shell integration, and completions
#
# Options:
#   --keep-config    Keep ~/.pjmai config directory (projects are preserved)
#   --dry-run        Show what would be removed without removing anything
#   --help           Show this help message

set -e

# Default configuration
KEEP_CONFIG=false
DRY_RUN=false
INSTALL_PREFIX="${HOME}/.local/bin"

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

dry_run_msg() {
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "${YELLOW}[DRY RUN]${NC} $1"
    fi
}

# Show help
show_help() {
    cat << EOF
PJMAI Uninstall Script

Usage: uninstall.sh [OPTIONS]

Options:
    --keep-config    Keep ~/.pjmai config directory (your projects list is preserved)
    --dry-run        Show what would be removed without actually removing anything
    --help           Show this help message

What gets removed:
    - Binary: ~/.local/bin/pjmai
    - Shell script: ~/.pjmai/source-pjm.sh
    - Config directory: ~/.pjmai/ (unless --keep-config)
    - Shell completions: ~/.zsh/completions/_pjmai (or bash/fish equivalents)
    - Lines added to ~/.zshrc or ~/.bashrc (with backup created)

EOF
    exit 0
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --keep-config)
                KEEP_CONFIG=true
                shift
                ;;
            --dry-run)
                DRY_RUN=true
                shift
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

# Remove a file with dry-run support
remove_file() {
    local file="$1"
    if [[ -f "$file" ]]; then
        if [[ "$DRY_RUN" == "true" ]]; then
            dry_run_msg "Would remove: $file"
        else
            rm -f "$file"
            success "Removed: $file"
        fi
    else
        info "Already absent: $file"
    fi
}

# Remove a directory with dry-run support
remove_dir() {
    local dir="$1"
    if [[ -d "$dir" ]]; then
        if [[ "$DRY_RUN" == "true" ]]; then
            dry_run_msg "Would remove directory: $dir"
        else
            rm -rf "$dir"
            success "Removed directory: $dir"
        fi
    else
        info "Already absent: $dir"
    fi
}

# Remove pjmai binary
remove_binary() {
    info "Removing pjmai binary..."
    remove_file "${INSTALL_PREFIX}/pjmai"
}

# Remove shell integration script
remove_shell_script() {
    info "Removing shell integration script..."
    remove_file "${HOME}/.pjmai/source-pjm.sh"
}

# Remove config directory
remove_config() {
    if [[ "$KEEP_CONFIG" == "true" ]]; then
        info "Keeping config directory (~/.pjmai) as requested"
        return
    fi

    local config_dir="${HOME}/.pjmai"
    if [[ -d "$config_dir" ]]; then
        # Check if there's a config.toml with projects
        if [[ -f "${config_dir}/config.toml" ]]; then
            local project_count
            project_count=$(grep -c '^\[\[project\]\]' "${config_dir}/config.toml" 2>/dev/null || echo "0")
            if [[ "$project_count" -gt 0 ]]; then
                warn "Config contains $project_count project(s). These will be lost!"
                if [[ "$DRY_RUN" != "true" ]]; then
                    read -p "Remove config directory? [y/N] " -n 1 -r
                    echo
                    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                        info "Keeping config directory"
                        return
                    fi
                fi
            fi
        fi
        remove_dir "$config_dir"
    else
        info "Config directory not found: $config_dir"
    fi
}

# Remove shell completions
remove_completions() {
    info "Removing shell completions..."

    # Zsh completions
    remove_file "${HOME}/.zsh/completions/_pjmai"

    # Bash completions
    remove_file "${HOME}/.local/share/bash-completion/completions/pjmai"

    # Fish completions
    remove_file "${HOME}/.config/fish/completions/pjmai.fish"
}

# Clean up shell rc file
clean_rc_file() {
    local rc_file="$1"
    local rc_name="$2"

    if [[ ! -f "$rc_file" ]]; then
        info "No $rc_name found at $rc_file"
        return
    fi

    # Check if our markers exist
    if ! grep -q "PJMAI" "$rc_file" 2>/dev/null; then
        info "No PJMAI entries found in $rc_name"
        return
    fi

    info "Cleaning PJMAI entries from $rc_name..."

    if [[ "$DRY_RUN" == "true" ]]; then
        dry_run_msg "Would remove PJMAI-related lines from $rc_file"
        echo "  Lines that would be removed:"
        grep -n "PJMAI\|source.*pjm\|source.*source-pjm" "$rc_file" 2>/dev/null | sed 's/^/    /'
        return
    fi

    # Create backup
    local backup="${rc_file}.pjmai-backup.$(date +%Y%m%d-%H%M%S)"
    cp "$rc_file" "$backup"
    success "Created backup: $backup"

    # Remove PJMAI-related lines
    # This removes:
    # - Lines containing "# PJMAI" (our marker comments)
    # - Lines containing "source" and "pjm" (source-pjm.sh)
    # - Lines containing "source" and "source-pjm"
    # - The fpath line we added (if it mentions .zsh/completions and is near PJMAI comment)

    local temp_file="${rc_file}.tmp"

    # Use awk to remove PJMAI blocks (marker + following non-empty lines that are related)
    awk '
    /# PJMAI/ { skip=1; next }
    /source.*pjm/ { next }
    /source.*source-pjm/ { next }
    /fpath=.*\.zsh\/completions/ && skip { next }
    /autoload.*compinit/ && skip { next }
    /^[[:space:]]*$/ { skip=0 }
    { skip=0; print }
    ' "$rc_file" > "$temp_file"

    # Also remove any standalone source-pjm lines that might not have markers
    sed -i '' '/source.*source-pjm\.sh/d' "$temp_file" 2>/dev/null || \
    sed -i '/source.*source-pjm\.sh/d' "$temp_file" 2>/dev/null || true

    # Remove empty lines at end of file (cleanup)
    sed -i '' -e :a -e '/^\n*$/{$d;N;ba' -e '}' "$temp_file" 2>/dev/null || true

    mv "$temp_file" "$rc_file"
    success "Cleaned $rc_name"
}

# Clean up shell configuration files
clean_shell_config() {
    info "Cleaning shell configuration files..."

    # Clean zshrc
    clean_rc_file "${HOME}/.zshrc" ".zshrc"

    # Clean bashrc
    clean_rc_file "${HOME}/.bashrc" ".bashrc"

    # Clean bash_profile (macOS)
    clean_rc_file "${HOME}/.bash_profile" ".bash_profile"

    # Clean fish config
    clean_rc_file "${HOME}/.config/fish/config.fish" "config.fish"
}

# Print summary
print_summary() {
    echo ""
    if [[ "$DRY_RUN" == "true" ]]; then
        echo "============================================"
        warn "DRY RUN - No changes were made"
        echo "============================================"
        echo ""
        echo "Run without --dry-run to actually uninstall."
    else
        echo "============================================"
        success "PJMAI uninstalled!"
        echo "============================================"
        echo ""
        echo "Restart your shell or run:"
        echo "    exec \$SHELL"
        echo ""
        if [[ "$KEEP_CONFIG" == "true" ]]; then
            echo "Your project config was preserved at ~/.pjmai/config.toml"
            echo "To reinstall later, your projects will still be there."
        fi
    fi
    echo ""
}

# Main uninstall flow
main() {
    echo ""
    echo "============================================"
    info "PJMAI Uninstaller"
    echo "============================================"
    echo ""

    parse_args "$@"

    if [[ "$DRY_RUN" == "true" ]]; then
        warn "DRY RUN MODE - showing what would be removed"
        echo ""
    fi

    remove_binary
    remove_completions
    clean_shell_config
    remove_shell_script
    remove_config
    print_summary
}

main "$@"
