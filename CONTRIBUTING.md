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

> This project ships with [`flatc`](https://github.com/google/flatbuffers) binaries for `Windows`, `macOS` and `Linux`, if you encounter
any issues during the build process, please open up an issue.

### 1. Clone the project:

```bash
git clone --recurse-submodules https://github.com/ev3nvy/rman-rs.git
```

> Note the `--recurse-submodules` part, which is required beacuse the schema in the linked submodule
is part of the build process.

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
