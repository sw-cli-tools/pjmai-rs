Output demo/full-workflow.gif
Output demo/full-workflow.mp4

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"
Set TypingSpeed 40ms

Env PJM_CONFIG_DIR "/tmp/pjm1-demo"
Env PATH "/Users/mike/github/wrightmikea/pjm1/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# =========================================="
Enter
Type "# PJM1 - Project Management CLI Demo"
Enter
Type "# =========================================="
Enter
Sleep 1s

Type "# Step 1: Source shell integration"
Enter
Sleep 300ms
Type "source ./source-pjm.sh"
Enter
Sleep 500ms

Type "# Step 2: Add projects with adpj"
Enter
Sleep 300ms
Type "adpj webapp -f /tmp/projects/webapp"
Enter
Sleep 400ms
Type "adpj backend -f /tmp/projects/backend"
Enter
Sleep 400ms
Type "adpj scripts -f /tmp/projects/scripts"
Enter
Sleep 500ms

Type "# Step 3: List projects with lspj"
Enter
Sleep 300ms
Type "lspj"
Enter
Sleep 1s

Type "# Step 4: Switch projects with chpj"
Enter
Sleep 300ms
Type "chpj backend"
Enter
Sleep 400ms
Type "pwd"
Enter
Sleep 500ms

Type "# Fuzzy matching works too"
Enter
Type "chpj scr"
Enter
Sleep 400ms
Type "pwd"
Enter
Sleep 500ms

Type "# Step 5: Show current project"
Enter
Sleep 300ms
Type "shpj"
Enter
Sleep 500ms
Type "prpj"
Enter
Sleep 500ms

Type "# Step 6: Remove a project with rmpj"
Enter
Sleep 300ms
Type "rmpj scripts"
Enter
Sleep 400ms
Type "lspj"
Enter
Sleep 1s

Type "# Step 7: View all aliases"
Enter
Sleep 300ms
Type "hlpj"
Enter
Sleep 2s
