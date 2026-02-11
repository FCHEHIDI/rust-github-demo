# 🦀 rust-github-demo

Demo project for Rust development with GitHub Codespaces.

## 🚀 Getting Started with Codespaces

1. Click the **Code** button (green button at the top)
2. Select the **Codespaces** tab
3. Click **Create codespace on main**
4. Wait for the container to build (first time takes a few minutes)
5. You're ready to code! 🎉

## 🛠️ Development

Once in Codespaces, try these commands:

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test

# Check code with clippy (linter)
cargo clippy

# Format code
cargo fmt

# Build optimized release version
cargo build --release
```

## 📦 What's Included

- ✅ Pre-configured Dev Container for Rust
- ✅ Rust Analyzer for IDE support
- ✅ Clippy for linting
- ✅ LLDB debugger support
- ✅ GitHub CLI included
- ✅ TOML syntax highlighting
- ✅ Crates.io integration

## 📝 Project Structure

```
rust-github-demo/
├── .devcontainer/
│   └── devcontainer.json    # Codespaces configuration
├── src/
│   └── main.rs              # Main source file
├── Cargo.toml               # Project manifest
├── .gitignore              # Git ignore rules for Rust
└── README.md               # This file
```

## 🎯 Example Code

The project includes a simple Fibonacci calculator to demonstrate:
- Basic Rust syntax
- Functions
- Pattern matching
- Unit tests

## 🔧 Customization

To add dependencies, edit `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
tokio = { version = "1", features = ["full"] }
```

Then run `cargo build` to install them.

## 📚 Learn More

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [GitHub Codespaces Docs](https://docs.github.com/codespaces)

---

Made with ❤️ and 🦀