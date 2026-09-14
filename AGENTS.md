# AI Project Contract

This file is binding for AI coding agents working in this repository.

## Before editing

- Read this file, README, CONTRIBUTING, relevant docs, and `git status`/recent log.
- Confirm the task, acceptance criteria, target branch, and files in scope.
- Do not guess kernel API, license text, or release intent.
- Do not work directly on `main` unless the owner explicitly authorizes an emergency fix.
- Userspace (`src/`, `tests/`) is the current product. Kernel / Rust-for-Linux work stays out of this tree until a later phase.

## During editing

- Make the smallest coherent change.
- Keep behavior, tests, docs, and user-facing copy consistent in one PR.
- Do not introduce unrelated cleanup, generated artifacts, secrets, debug code, or dependency churn.
- The I/O contract is Linux `drivers/char/mem.c`: `read_null` returns 0, `write_null` returns `count`, `null_lseek` sets position to 0. Do not invent a friendlier seek.
- If this repository is public: do not commit `.env`, live keys, or household/private domains (scan git history, not only HEAD). Private repos are exempt.

## Before commit / push / merge

- Review the complete diff and scan for secrets, conflict markers, TODO/FIXME, debug output, and accidental files.
- Run `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, and `cargo test --locked`.
- New failures block the commit. Do not hide failures as warnings.
- Use a focused Conventional Commit: `type(scope): imperative summary`.
- Push a branch and use a PR. Do not bypass required checks.
- Update docs and `[Unreleased]` when behavior or user-facing output changes.
- Report exactly what was verified; never call an unverified change complete.

## Stop conditions

Stop and ask when scope, destructive action, public behavior, credentials, or factual copy is unclear. Block when security, correctness, required checks, or release prerequisites fail. Do not `insmod` / `rmmod` / build against a live kernel without an explicit owner order.
