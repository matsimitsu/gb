# Settings
Output demo.gif

# Setup
Hide

# Build and alias
Type 'cargo build' Enter
Type 'alias gb="$(pwd)/target/debug/gb"' Enter

# Make temp repository
Type "cd $(mktemp -d)" Enter
Type "git init demo-repo" Enter
Type "cd demo-repo" Enter
Type "touch README.md" Enter
Type "git add README.md" Enter
Type 'git commit -m "Initial commit"' Enter
Type "git switch -c feature/login" Enter
Type "git switch -c feature/signup" Enter
Type "git switch -c bugfix/header" Enter
Type "git switch main" Enter
Type 'clear'
Enter
Show

# Demo
Type "git status" Enter
Sleep 1s
Type "gb"
Sleep 1s
Enter
Sleep 2s

# Show navigation
Type "j" Sleep 500ms
Type "j" Sleep 500ms
Type "k" Sleep 500ms
Sleep 1s

# Switch branches
Type "j" Sleep 500ms Enter
Sleep 1s
Type "git status" Enter
Sleep 1s
Type "gb" Enter

# Show final state
Sleep 2s

# Exit
Type "q" Enter
