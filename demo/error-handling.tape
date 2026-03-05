# Error Handling Demo
# This tape demonstrates error messages using shell aliases
# Run ./demo/setup-demo.sh before recording this tape

Output demo/error-handling.gif
Output demo/error-handling.mp4

Set Shell "bash"
Set FontSize 14
Set Width 900
Set Height 400
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJMAI_CONFIG_DIR "/tmp/pjmai-error-demo"
Env PATH "/Users/mike/github/sw-cli-tools/pjmai-rs/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# Source shell integration"
Enter
Sleep 300ms

Type "source ./source-pjm.sh"
Enter
Sleep 500ms

Type "# Show existing projects"
Enter
Sleep 500ms

Type "lspj"
Enter
Sleep 1s

Type "# Try to add a duplicate project name"
Enter
Sleep 500ms

Type "adpj existing -f /tmp/error-demo-project"
Enter
Sleep 1.5s

Type "# Try to add with nonexistent path"
Enter
Sleep 500ms

Type "adpj bad -f /nonexistent/path"
Enter
Sleep 1.5s

Type "# Try to change to unknown project"
Enter
Sleep 500ms

Type "chpj unknown"
Enter
Sleep 1.5s

Type "# Clear error messages help diagnose issues"
Enter
Sleep 2s
