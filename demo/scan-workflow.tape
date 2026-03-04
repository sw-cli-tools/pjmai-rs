# PJMAI Scan Workflow Demo
# This tape demonstrates scanning for git repositories and managing projects
# Run ./demo/setup-demo.sh before recording this tape

Output demo/scan-workflow.gif
Output demo/scan-workflow.mp4

Set Shell "bash"
Set FontSize 14
Set Width 900
Set Height 500
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJMAI_CONFIG_DIR "/tmp/pjmai-demo"
Env PATH "/Users/mike/github/wrightmikea/pjmai/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# PJMAI - Discover all your git repositories"
Enter
Sleep 500ms

Type "source ./source-pjm.sh"
Enter
Sleep 500ms

Type "# Scan a directory tree for git repos (dry-run)"
Enter
Sleep 500ms

Type "scpj /tmp/pjmai-demo-repos --dry-run"
Enter
Sleep 2s

Type "# Add all discovered repositories"
Enter
Sleep 500ms

Type "scpj /tmp/pjmai-demo-repos --add-all"
Enter
Sleep 2s

Type "# List all projects"
Enter
Sleep 500ms

Type "lspj"
Enter
Sleep 2s

Type "# Rename an auto-generated nickname"
Enter
Sleep 500ms

Type "mvpj webapp webapp-prod"
Enter
Sleep 1s

Type "# Remove unwanted projects"
Enter
Sleep 500ms

Type "rmpj old-experiment"
Enter
Sleep 1s

Type "# Final project list"
Enter
Sleep 500ms

Type "lspj"
Enter
Sleep 2s

Type "# Switch to a project instantly"
Enter
Sleep 500ms

Type "chpj webapp-prod"
Enter
Sleep 1s

Type "pwd"
Enter
Sleep 1.5s
