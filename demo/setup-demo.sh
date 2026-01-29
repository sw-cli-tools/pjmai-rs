#!/bin/bash
# Setup script for VHS demos
# Creates temp directories and config files

# Create demo directories
mkdir -p /tmp/pjm1-demo
mkdir -p /tmp/projects/{webapp,backend,scripts}
mkdir -p /tmp/demo-projects/{project-a,project-b,project-c,temp-project}
mkdir -p /tmp/error-demo-project

# Create empty config for basic-workflow and project-management demos
cat > /tmp/pjm1-demo/config.toml << 'EOF'
version = "0.1.0"
current_project = ""
project = []
EOF

# Create config with existing project for error-handling demo
mkdir -p /tmp/pjm1-error-demo
cat > /tmp/pjm1-error-demo/config.toml << 'EOF'
version = "0.1.0"
current_project = "existing"

[[project]]
name = "existing"

[project.action]
file_or_dir = "/tmp/error-demo-project"
EOF

echo "Demo environment ready!"
