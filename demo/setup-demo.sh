#!/bin/bash
# Setup script for VHS demos
# Creates temp directories and config files

# Create demo directories
mkdir -p /tmp/pjmai-demo
mkdir -p /tmp/projects/{webapp,backend,scripts}
mkdir -p /tmp/demo-projects/{project-a,project-b,project-c,temp-project}
mkdir -p /tmp/error-demo-project

# Create git repositories for scan-workflow demo
SCAN_DEMO_DIR="/tmp/pjmai-demo-repos"
rm -rf "$SCAN_DEMO_DIR"
mkdir -p "$SCAN_DEMO_DIR"

# Create fake git repos with remotes
create_git_repo() {
    local dir="$1"
    local remote="$2"
    mkdir -p "$dir"
    git -C "$dir" init -q
    if [[ -n "$remote" ]]; then
        git -C "$dir" remote add origin "$remote"
    fi
    touch "$dir/README.md"
}

# GitHub repos (different orgs)
create_git_repo "$SCAN_DEMO_DIR/work/webapp" "git@github.com:acme-corp/webapp.git"
create_git_repo "$SCAN_DEMO_DIR/work/api-server" "git@github.com:acme-corp/api-server.git"
create_git_repo "$SCAN_DEMO_DIR/personal/dotfiles" "git@github.com:developer/dotfiles.git"
create_git_repo "$SCAN_DEMO_DIR/personal/side-project" "git@github.com:developer/side-project.git"

# Local repo (no remote)
create_git_repo "$SCAN_DEMO_DIR/experiments/old-experiment" ""

# Create empty config for basic-workflow and project-management demos
cat > /tmp/pjmai-demo/config.toml << 'EOF'
version = "0.1.0"
current_project = ""
project = []
EOF

# Create config with existing project for error-handling demo
mkdir -p /tmp/pjmai-error-demo
cat > /tmp/pjmai-error-demo/config.toml << 'EOF'
version = "0.1.0"
current_project = "existing"

[[project]]
name = "existing"

[project.action]
file_or_dir = "/tmp/error-demo-project"
EOF

echo "Demo environment ready!"
