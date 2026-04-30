# Contributing to Gust

Thank you for your interest in contributing to Gust!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/gust`
3. Install Rust: <https://rustup.rs>
4. Install wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
5. Build the project: `make build`

## Development

```bash
make build   # Build WASM
make demo    # Build and start the demo
```

## Making Changes

- Keep changes focused — one feature or fix per PR
- Write tests for new logic (`cargo test`)
- Ensure TypeScript types are correct
- Follow existing code style

## Submitting a PR

1. Create a branch: `git checkout -b feature/your-feature`
2. Make your changes
3. Run `cargo test`
4. Open a PR and fill out the template

## Reporting Bugs

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md).

## License

By contributing to Gust, you agree that your contributions will be licensed under the AGPL-3.0.
