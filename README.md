# GB - Git Branch Manager

GB is a terminal user interface (TUI) application for managing Git branches in a more intuitive way. It provides an interactive interface to view, and switch your Git branches efficiently from the command line.

## Features

- Interactive TUI using Ratatui and Crossterm
- View all local and remote branches
- Switch between branches quickly
- Real-time Git repository status
- Keyboard-driven interface

## Installation

With Rust's package manager Cargo, you can install GB directly:

```bash
cargo install gb
```

Or build from source:

```bash
git clone https://github.com/your-username/gb.git
cd gb
cargo build --release
```

The binary will be available in `target/release/gb`

## Usage

Simply run `gb` in any Git repository:

```bash
gb
```

### Key Bindings

- `↑/↓`: Navigate through branches
- `Enter`: Switch to selected branch
- `q`: Quit the application

## Requirements

- Rust 1.75 or later
- Git (2.0 or later)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
