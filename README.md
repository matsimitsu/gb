# GB - Git Branch

I needed something to quickly switch between (recent) branches, and using `lg` (LazyGit) then type `3` and then select the branch was doable, but it could be done quicker, so I made `gb`.

It shows the last 10 branches, sorted by date (descending), and you can navigate using `↑/↓/j/k` to select the branch you'd like to check out, and press `enter` to do so.

![Demo of GB in action](demo.gif)

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

- `↑/↓/j/k`: Navigate through branches
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
