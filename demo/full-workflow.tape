# Full PJM1 Workflow Demo
# Shows installation, adding, switching, and removing projects
# Run ./demo/setup-demo.sh before recording this tape

Output demo/full-workflow.gif
Output demo/full-workflow.mp4

Set Shell "zsh"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"
Set TypingSpeed 40ms

Env PJM_CONFIG_DIR "/tmp/pjm1-demo"
Env PATH "/Users/mike/github/wrightmikea/pjm1/target/release:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"

Type "# =========================================="
Enter
Type "# PJM1 - Project Management CLI"
Enter
Type "# =========================================="
Enter
Sleep 1s

Type "# STEP 1: Installation"
Enter
Type "# Add these lines to your ~/.zshrc (Mac) or ~/.bashrc (Linux):"
Enter
Sleep 500ms

Type "cat << 'INSTALL'"
Enter
Type "# PJM1 project management"
Enter
Type 'export PATH="$HOME/.local/bin:$PATH"'
Enter
Type "source /path/to/pjm1/source-pjm.sh"
Enter
Type "INSTALL"
Enter
Sleep 2s

Type "# For this demo, we source the shell integration directly:"
Enter
Sleep 300ms

Type "source ./source-pjm.sh"
Enter
Sleep 700ms

Type "# =========================================="
Enter
Type "# STEP 2: Adding Projects"
Enter
Type "# =========================================="
Enter
Sleep 500ms

Type "# Add a web application project"
Enter
Type "adpj webapp -f /tmp/projects/webapp"
Enter
Sleep 800ms

Type "# Add a backend API project"
Enter
Type "adpj backend -f /tmp/projects/backend"
Enter
Sleep 800ms

Type "# Add a scripts project"
Enter
Type "adpj scripts -f /tmp/projects/scripts"
Enter
Sleep 800ms

Type "# =========================================="
Enter
Type "# STEP 3: Listing Projects"
Enter
Type "# =========================================="
Enter
Sleep 500ms

Type "# List all projects (current project shows '>')"
Enter
Type "lspj"
Enter
Sleep 1.5s

Type "# =========================================="
Enter
Type "# STEP 4: Switching Projects"
Enter
Type "# =========================================="
Enter
Sleep 500ms

Type "# Switch to webapp project"
Enter
Type "chpj webapp"
Enter
Sleep 800ms

Type "pwd"
Enter
Sleep 500ms

Type "# Switch to backend (changes directory)"
Enter
Type "chpj backend"
Enter
Sleep 800ms

Type "pwd"
Enter
Sleep 500ms

Type "# Fuzzy matching works too - 'scr' matches 'scripts'"
Enter
Type "chpj scr"
Enter
Sleep 800ms

Type "pwd"
Enter
Sleep 1s

Type "# =========================================="
Enter
Type "# STEP 5: Show Current Project"
Enter
Type "# =========================================="
Enter
Sleep 500ms

Type "# Show detailed info about current project"
Enter
Type "shpj"
Enter
Sleep 1s

Type "# Get project name (useful for shell prompts)"
Enter
Type "prpj"
Enter
Sleep 1s

Type "# =========================================="
Enter
Type "# STEP 6: Removing Projects"
Enter
Type "# =========================================="
Enter
Sleep 500ms

Type "# Remove the scripts project"
Enter
Type "rmpj scripts"
Enter
Sleep 800ms

Type "# Verify it is gone"
Enter
Type "lspj"
Enter
Sleep 1.5s

Type "# =========================================="
Enter
Type "# STEP 7: View Available Aliases"
Enter
Type "# =========================================="
Enter
Sleep 500ms

Type "hlpj"
Enter
Sleep 2s

Type "# Done! Projects managed efficiently."
Enter
Sleep 1s
