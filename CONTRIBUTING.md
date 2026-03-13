# Contributing to nanoaios

Thanks for your interest in contributing to `nanoaios`.
This project aims to keep an AIOS-native core simple, reliable, and production-oriented.

## Principles

- Keep the core minimal and composable.
- Preserve clear layer boundaries (`Kernel -> Runtime -> Provider -> API`).
- Prefer explicit behavior over hidden magic.
- Ship changes that are testable and observable.

## Development Setup

Requirements:

- Rust stable toolchain
- Linux/macOS shell environment

Run locally:

```bash
cargo run -- init
cargo run -- start
```

Smoke test:

```bash
curl -s http://127.0.0.1:4242/healthz
cargo run -- chat "smoke test"
```

## Before You Open a PR

Please ensure all checks pass:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

For runtime-related changes, also run:

```bash
cargo run -- init --force
cargo run -- chat "runtime check"
```

## Commit Guidelines

- Use small, atomic commits.
- Write concise commit messages with intent.
- Suggested prefixes:
  - `feat:` new functionality
  - `fix:` bug fix
  - `docs:` documentation updates
  - `refactor:` non-behavioral code improvements
  - `test:` test-only changes

Example:

```text
feat(runtime): add provider timeout handling
```

## Pull Request Guidelines

A good PR should include:

- What changed
- Why it changed
- How it was tested
- Any follow-up work (if needed)

Keep PR scope focused. If a change touches multiple concerns, split it into separate PRs.

## Reporting Issues

When opening an issue, include:

- Environment (`rustc --version`, OS)
- Reproduction steps
- Expected behavior
- Actual behavior
- Relevant logs/output

## Code of Conduct

Be respectful and constructive.
Harassment, discrimination, or personal attacks are not tolerated.
