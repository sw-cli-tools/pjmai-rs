# Basic PJM1 Workflow Demo
# This tape demonstrates the core workflow using shell aliases
# Run ./demo/setup-demo.sh before recording this tape

Output demo/basic-workflow.gif

Set Shell "zsh"
Set FontSize 14
Set Width 800
Set Height 400
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJM_CONFIG_DIR "/tmp/pjm1-demo"
Env PATH "/Users/mike/github/wrightmikea/pjm1/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# PJM1 - Project Management CLI"
Enter
Sleep 500ms

Type "# First, source the shell integration"
Enter
Sleep 300ms

Type "source ./source-pjm.sh"
Enter
Sleep 500ms

Type "# Add project directories using 'adpj' alias"
Enter
Sleep 500ms

Type "adpj webapp -f /tmp/projects/webapp"
Enter
Sleep 1s

Type "adpj backend -f /tmp/projects/backend"
Enter
Sleep 1s

Type "# List all projects with 'lspj'"
Enter
Sleep 500ms

Type "lspj"
Enter
Sleep 1.5s

Type "# Show current project with 'shpj'"
Enter
Sleep 500ms

Type "shpj"
Enter
Sleep 1.5s

Type "# Get project name for prompt with 'prpj'"
Enter
Sleep 500ms

Type "prpj"
Enter
Sleep 1s

Type "# View all aliases with 'hlpj'"
Enter
Sleep 500ms

Type "hlpj"
Enter
Sleep 2s
