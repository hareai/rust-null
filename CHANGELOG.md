# Changelog

All notable changes are documented here.

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), versioned by [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Userspace `NullDevice` matching Linux `/dev/null` (`read` EOF, `write` discard, `lseek` always 0).
- Contract tests against `drivers/char/mem.c` helpers `read_null`, `write_null`, and `null_lseek`.
- `examples/discard` stdin sink.
