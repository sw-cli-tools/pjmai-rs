# Basic PJM1 Workflow Demo
# This tape demonstrates the core workflow: add, list, change, and show commands
# Run ./demo/setup-demo.sh before recording this tape

Output demo/basic-workflow.gif

Set Shell "bash"
Set FontSize 14
Set Width 800
Set Height 400
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJM_CONFIG_DIR "/tmp/pjm1-demo"

Type "# PJM1 - Project Management CLI"
Enter
Sleep 1s

Type "# Add a project directory"
Enter
Sleep 500ms

Type "./target/release/pjm1 add -p webapp -f /tmp/projects/webapp"
Enter
Sleep 1s

Type "./target/release/pjm1 add -p backend -f /tmp/projects/backend"
Enter
Sleep 1s

Type "# List all projects"
Enter
Sleep 500ms

Type "./target/release/pjm1 list"
Enter
Sleep 1.5s

Type "# Show current project details"
Enter
Sleep 500ms

Type "./target/release/pjm1 show"
Enter
Sleep 1.5s

Type "# Get project name for shell prompt"
Enter
Sleep 500ms

Type "./target/release/pjm1 prompt"
Enter
Sleep 1.5s

Type "# View available aliases"
Enter
Sleep 500ms

Type "./target/release/pjm1 aliases"
Enter
Sleep 2s
