# Error Handling Demo
# This tape demonstrates error handling: duplicate add, nonexistent path, unknown project
# Run ./demo/setup-demo.sh before recording this tape

Output demo/error-handling.gif

Set Shell "bash"
Set FontSize 14
Set Width 900
Set Height 400
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJM_CONFIG_DIR "/tmp/pjm1-error-demo"

Type "# PJM1 Error Handling"
Enter
Sleep 1s

Type "# Show existing projects"
Enter
Sleep 500ms

Type "./target/release/pjm1 list"
Enter
Sleep 1s

Type "# Try to add a duplicate project name"
Enter
Sleep 500ms

Type "./target/release/pjm1 add -p existing -f /tmp/error-demo-project"
Enter
Sleep 1.5s

Type "# Try to add with nonexistent path"
Enter
Sleep 500ms

Type "./target/release/pjm1 add -p bad -f /nonexistent/path"
Enter
Sleep 1.5s

Type "# Try to change to unknown project"
Enter
Sleep 500ms

Type "./target/release/pjm1 change -p unknown"
Enter
Sleep 1.5s

Type "# Clear error messages help diagnose issues"
Enter
Sleep 2s
