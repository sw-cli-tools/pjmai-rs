# PJMAI Prompt Integration Demo
# Shows project indicator in shell prompt with push/pop stack

Output demo/prompt-integration.gif
Output demo/prompt-integration.mp4

Set Shell "bash"
Set FontSize 14
Set Width 900
Set Height 500
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJMAI_CONFIG_DIR "/tmp/pjmai-prompt-demo"
Env PATH "/Users/mike/github/sw-cli-tools/pjmai-rs/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# PJMAI - Shell Prompt Integration"
Enter
Sleep 500ms

Type "# Source shell integration and prompt setup"
Enter
Sleep 300ms

Type "source ./source-pjm.sh"
Enter
Sleep 500ms

Type "source /tmp/pjmai-prompt-demo/setup-prompt.sh"
Enter
Sleep 500ms

Type "# Add some projects"
Enter
Sleep 300ms

Type "adpj webapp -f /tmp/projects/webapp"
Enter
Sleep 500ms

Type "adpj api -f /tmp/projects/api"
Enter
Sleep 500ms

Type "adpj config -f /tmp/projects/config"
Enter
Sleep 500ms

Type "# Switch to webapp - prompt shows project"
Enter
Sleep 500ms

Type "chpj webapp"
Enter
Sleep 1s

Type "# Push to api - stack depth appears"
Enter
Sleep 500ms

Type "pspj api"
Enter
Sleep 1s

Type "# Push again to config - stack grows"
Enter
Sleep 500ms

Type "pspj config"
Enter
Sleep 1s

Type "# Show current project and stack"
Enter
Sleep 500ms

Type "shpj"
Enter
Sleep 1.5s

Type "# Pop back to api"
Enter
Sleep 500ms

Type "popj"
Enter
Sleep 1s

Type "# Pop back to webapp"
Enter
Sleep 500ms

Type "popj"
Enter
Sleep 1s

Type "# Stack empty - clean prompt"
Enter
Sleep 500ms

Type "shpj"
Enter
Sleep 1.5s
