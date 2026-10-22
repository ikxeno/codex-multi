# Contributing

Keep changes focused and describe the problem they solve. For changes to CLI
behavior, include an example command and its expected result.

## Local setup

Install [rustup](https://rustup.rs/), then build and test from the repository
root. Rustup uses the toolchain and components in `rust-toolchain.toml`.

```sh
cargo build --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

Run `cargo fmt --all` to apply formatting. Commit `Cargo.lock` when dependencies
change; this is an application rather than a library.

Add tests for observable behavior such as arguments, exit codes, and session
handling. Keep dependencies limited to what the implementation uses.

Before submitting a pull request, describe the change and how it was tested.
Follow the [code of conduct](CODE_OF_CONDUCT.md).
