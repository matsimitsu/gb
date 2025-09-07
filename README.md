# GB - Git Branch Manager

GB is a terminal user interface (TUI) application for managing Git branches in a more intuitive way. It provides an interactive interface to view, switch, and manage your Git branches efficiently from the command line.

![Demo of GB in action](demo.gif)

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

## Demo Generation

The demo GIF is generated using [VHS](https://github.com/charmbracelet/vhs). If you have VHS installed, you can regenerate the demo by running:

```bash
vhs demo.tape
```
