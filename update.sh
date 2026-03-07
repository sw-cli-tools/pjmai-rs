# PJMAI-RS Update Script
# Quick reinstall for development - builds and copies binary + shell script
#
# Usage:
#   source update.sh              # Build, install, and reload shell integration
#   source update.sh --prefix DIR # Install to custom location

# Guard: must be sourced, not executed
if [[ "${BASH_SOURCE[0]:-$0}" == "${0}" ]] && [[ -z "$ZSH_EVAL_CONTEXT" ]]; then
    echo "Error: Do not run this script, source it instead:"
    echo "  source update.sh"
    exit 1
fi

# Default locations (same as install.sh)
INSTALL_PREFIX="${HOME}/.local/bin"
PJMAI_DIR="${HOME}/.pjmai"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

info() { echo -e "${BLUE}==>${NC} $1"; }
success() { echo -e "${GREEN}==>${NC} $1"; }
error() { echo -e "${RED}Error:${NC} $1"; }

# Parse --prefix argument
_pjm_update_args=("$@")
while [[ $# -gt 0 ]]; do
    case "$1" in
        --prefix)
            INSTALL_PREFIX="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done
set -- "${_pjm_update_args[@]}"
unset _pjm_update_args

# Get script directory (works when sourced)
if [[ -n "$ZSH_VERSION" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${(%):-%x}")" && pwd)"
else
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
fi

info "Building pjmai-rs..."
(
    cd "$SCRIPT_DIR"
    cargo build --release 2>&1 | grep -E '^(   Compiling|    Finished)' | sed 's/^/    /' || true
)

if [[ ! -f "$SCRIPT_DIR/target/release/pjmai-rs" ]]; then
    error "Build failed"
    return 1
fi

info "Installing binary to ${INSTALL_PREFIX}/pjmai-rs..."
mkdir -p "$INSTALL_PREFIX"
cp "$SCRIPT_DIR/target/release/pjmai-rs" "$INSTALL_PREFIX/pjmai-rs"
chmod +x "$INSTALL_PREFIX/pjmai-rs"

info "Installing source-pjm.sh to ${PJMAI_DIR}/..."
mkdir -p "$PJMAI_DIR"
cp "$SCRIPT_DIR/source-pjm.sh" "$PJMAI_DIR/source-pjm.sh"

# Show version
VERSION=$("$INSTALL_PREFIX/pjmai-rs" --version 2>&1 | head -1)
success "Updated: ${VERSION}"

# Clear project stack (reinstall invalidates stack state)
"$INSTALL_PREFIX/pjmai-rs" stack clear -y 2>/dev/null && info "Project stack cleared" || true

info "Reloading shell integration..."
source "$PJMAI_DIR/source-pjm.sh"
success "Shell integration reloaded!"
