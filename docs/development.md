# Development Workflow

## Branches

`main` is the protected integration branch. Work on `feat/*`, `fix/*`, `refactor/*`, `docs/*`, `test/*`, or `ci/*`.

## Local gate

Before opening a PR:

```bash
./scripts/governance-check.sh
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

Optional, when `linux-headers-$(uname -r)` is installed:

```bash
make -C kernel
```

Do not `make -C kernel load` unless the owner asked to test `/dev/rust-null` on that machine.

Public GitHub repos: `git config hermes.public true`. Do not commit `.env`, live keys, or household/private domains (history counts). Private repos are exempt.

## PR gate

A PR must be focused, documented, tested where behavior changes, reviewed by the AI assistant, and green in CI. Solo development does not remove the gate; it replaces a second human reviewer with an explicit AI review and automated checks.

## Commit gate

Use `type(scope): imperative summary`; one logical change per commit. Keep history understandable and rollback-friendly.

## Merge gate

Only merge after all required checks pass. Prefer squash merge into `main`. Delete the topic branch after merge.
