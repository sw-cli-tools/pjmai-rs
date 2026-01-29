# Project Management Demo
# This tape demonstrates managing multiple projects using shell aliases
# Run ./demo/setup-demo.sh before recording this tape

Output demo/project-management.gif

Set Shell "zsh"
Set FontSize 14
Set Width 800
Set Height 450
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJM_CONFIG_DIR "/tmp/pjm1-demo"
Env PATH "/Users/mike/github/wrightmikea/pjm1/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# Source shell integration for aliases"
Enter
Sleep 300ms

Type "source ./source-pjm.sh"
Enter
Sleep 500ms

Type "# Add several projects with 'adpj'"
Enter
Sleep 500ms

Type "adpj proj-a -f /tmp/demo-projects/project-a"
Enter
Sleep 700ms

Type "adpj proj-b -f /tmp/demo-projects/project-b"
Enter
Sleep 700ms

Type "adpj proj-c -f /tmp/demo-projects/project-c"
Enter
Sleep 700ms

Type "adpj temp -f /tmp/demo-projects/temp-project"
Enter
Sleep 700ms

Type "# List all projects (sorted alphabetically)"
Enter
Sleep 500ms

Type "lspj"
Enter
Sleep 1.5s

Type "# Remove a project with 'rmpj'"
Enter
Sleep 500ms

Type "rmpj temp"
Enter
Sleep 1s

Type "# Verify it is gone"
Enter
Sleep 500ms

Type "lspj"
Enter
Sleep 2s
