# Project Management Demo
# This tape demonstrates managing multiple projects and removing them
# Run ./demo/setup-demo.sh before recording this tape

Output demo/project-management.gif

Set Shell "bash"
Set FontSize 14
Set Width 800
Set Height 450
Set Theme "Dracula"
Set TypingSpeed 50ms

Env PJM_CONFIG_DIR "/tmp/pjm1-demo"

Type "# Managing multiple projects with PJM1"
Enter
Sleep 1s

Type "# Add several projects"
Enter
Sleep 500ms

Type "./target/release/pjm1 add -p proj-a -f /tmp/demo-projects/project-a"
Enter
Sleep 700ms

Type "./target/release/pjm1 add -p proj-b -f /tmp/demo-projects/project-b"
Enter
Sleep 700ms

Type "./target/release/pjm1 add -p proj-c -f /tmp/demo-projects/project-c"
Enter
Sleep 700ms

Type "./target/release/pjm1 add -p temp -f /tmp/demo-projects/temp-project"
Enter
Sleep 700ms

Type "# List all projects (sorted alphabetically)"
Enter
Sleep 500ms

Type "./target/release/pjm1 list"
Enter
Sleep 1.5s

Type "# Remove the temporary project"
Enter
Sleep 500ms

Type "./target/release/pjm1 remove -p temp"
Enter
Sleep 1s

Type "# Verify it is gone"
Enter
Sleep 500ms

Type "./target/release/pjm1 list"
Enter
Sleep 2s
