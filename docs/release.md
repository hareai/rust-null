# Release Workflow

```text
focused PR → CI green → merge to main → finalize CHANGELOG → tag → publish → smoke test
```

## Checklist

- [ ] `main` is up to date and clean
- [ ] Required CI is green
- [ ] `[Unreleased]` is complete
- [ ] Version follows `vMAJOR.MINOR.PATCH`
- [ ] Breaking changes are called out
- [ ] Tag points to the reviewed commit
- [ ] `cargo test` still passes on the tagged commit
- [ ] Rollback target is known

A build completing is not enough: verify the published artifact or tagged tree with `cargo test`.
