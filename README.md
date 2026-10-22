# codex-multi

A Rust CLI for managing and switching between multiple Codex accounts.

This repository currently contains the initial project scaffold. The binary
supports `--help`. Account management, hotkeys, and terminal session switching
are planned.

## Requirements

Install Rust through [rustup](https://rustup.rs/). The project pins Rust 1.94.1
and the `clippy` and `rustfmt` components in `rust-toolchain.toml`.

## Run

```sh
cargo run --locked -- --help
```

Running without arguments also displays help.

## Build

```sh
cargo build --locked --release
```

The executable is written to `target/release/codex-multi`
(`target/release/codex-multi.exe` on Windows).

## Development

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

GitHub Actions checks formatting and Clippy, and tests and builds the project
on Linux, macOS, and Windows. Dependency updates are configured with Dependabot.

## Layout

```text
src/main.rs          Executable entry point
src/cli.rs           Command-line argument definition
tests/cli.rs         Binary integration tests
.github/             CI, dependency updates, and contribution templates
Cargo.toml           Package metadata and dependencies
Cargo.lock           Locked application dependencies
rust-toolchain.toml  Pinned development toolchain
```

## Scaffold

The project follows Cargo's binary package layout, with GitHub project
boilerplate inspired by [Rust GitHub Template](https://github.com/rust-github/template).
The scaffold has been adapted for this CLI; its initial runtime dependency is
[`clap`](https://github.com/clap-rs/clap).

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and
[CHANGELOG.md](CHANGELOG.md) for changes.

## License

[Apache License 2.0](LICENSE), copyright 2026 Ivan Khanas.
