# Contributing

If you find any bugs or issues when using this crate, feel free to create an issue or open a
pull request.

## Setting up your development environment

Prerequisites:
  - [rust](https://www.rust-lang.org)
  - [clippy](https://github.com/rust-lang/rust-clippy) (installed alongside rust if `rustup` was
  used for installation)
  - [rust-fmt](https://github.com/rust-lang/rustfmt) (installed alongside rust if `rustup` was
  used for installation)

### 1. Clone the project:

```bash
git clone https://github.com/ev3nvy/rman-rs.git
```

### 2. Make the desired changes.

### 3. Check if the code compiles:

```bash
cargo check
```

### 4. Run clippy:

```bash
cargo clippy
```

### 5. Run cargo-fmt

```bash
cargo fmt
```

### 6. Run tests

```bash
cargo test
cargo test --all-features
```

### 7. Open a Pull Request
