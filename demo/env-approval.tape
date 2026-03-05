# PJMAI Environment Approval Demo
# Shows secure handling of .pjmai.sh project environment files

Output demo/env-approval.gif
Output demo/env-approval.mp4

Set Shell "bash"
Set FontSize 14
Set Width 900
Set Height 500
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJMAI_CONFIG_DIR "/tmp/pjmai-env-demo"
Env PATH "/Users/mike/github/sw-cli-tools/pjmai-rs/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# PJMAI - Secure Project Environment Files"
Enter
Sleep 500ms

Type "source ./source-pjm.sh"
Enter
Sleep 500ms

Type "# Create a Python project with .pjmai.sh"
Enter
Sleep 300ms

Type "mkdir -p /tmp/pyproject && cd /tmp/pyproject"
Enter
Sleep 500ms

Type "echo 'export SRCROOT=$PWD' > .pjmai.sh"
Enter
Sleep 300ms

Type "echo 'export PYTHONPATH=$PWD/src' >> .pjmai.sh"
Enter
Sleep 500ms

Type "cat .pjmai.sh"
Enter
Sleep 1s

Type "# Register and switch to project"
Enter
Sleep 300ms

Type "adpj pyproj -f /tmp/pyproject"
Enter
Sleep 500ms

Type "cd /tmp && chpj pyproj"
Enter
Sleep 1s

Type "# Warning shown - file not yet approved"
Enter
Sleep 500ms

Type "# Inspect it first"
Enter
Sleep 300ms

Type "cat .pjmai.sh"
Enter
Sleep 1s

Type "# Looks safe - approve and source it"
Enter
Sleep 500ms

Type "srcpj"
Enter
Sleep 1s

Type "# Verify env vars are set"
Enter
Sleep 300ms

Type "echo SRCROOT=$SRCROOT"
Enter
Sleep 1s

Type "# Switch away and back - auto-sources now"
Enter
Sleep 500ms

Type "cd /tmp"
Enter
Sleep 300ms

Type "chpj pyproj"
Enter
Sleep 1s

Type "# No warning - approved file auto-sourced"
Enter
Sleep 500ms

Type "echo SRCROOT=$SRCROOT"
Enter
Sleep 1.5s
