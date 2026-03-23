# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is xrun

A Rust CLI tool that wraps `xcodebuild` to simplify running iOS/macOS unit tests from the terminal or CI. It pipes xcodebuild output through `xcpretty`, parses test results, and optionally generates PDF reports.

## Commands

This project uses [mise](https://mise.jdx.dev/) as a task runner:

```sh
mise build        # cargo build (debug)
mise release      # cargo build --release
mise test         # cargo test
mise lint         # cargo clippy --all-targets --all-features
mise fmt          # cargo fmt --all -- --check
mise check        # cargo check
mise doc          # cargo doc --no-deps
mise ios          # run xrun against the sample/ Xcode project
```

To run a single test: `cargo test <test_name>`

## Architecture

The binary is orchestrated through `main.rs` → `lib.rs` with these modules:

| Module | Role |
|--------|------|
| `args.rs` | Parses and validates CLI positional arguments |
| `validator.rs` | Handles `fail` and `generate-file` flags |
| `output.rs` | Spawns `xcodebuild` piped to `xcpretty`, reads stdout line by line |
| `validation_lines.rs` | Parses test result lines (✓/✗ markers) from xcpretty output |
| `results.rs` | Aggregates pass/fail counts and renders result tables |
| `pdf_report.rs` | Generates `results-xrun.pdf` on test failure when flag is set |
| `text_utils.rs` | HTML entity decoding and PDF text sanitization |
| `spinner.rs` | Progress spinner with elapsed time while tests run |
| `header.rs` | ASCII art banner rendered with `rascii_art` |
| `utils.rs` | Shared messaging utilities |

**Execution flow:** header → argument parsing → xcodebuild subprocess → line-by-line output parsing → result aggregation → optional PDF generation → exit code.

**CLI signature:**
```
xrun <extension> <path> <scheme> <platform_or_version> [device] [fail] [generate-file]
```
- `extension`: `project` or `workspace`
- `platform_or_version`: `macOS` or an iOS version string (e.g. `17.4`)
- `device`: iOS simulator version (iOS only, e.g. `15`)
- `fail`: exit non-zero on test failures
- `generate-file`: write `results-xrun.pdf` on failure

## Key details

- **Edition 2024**, MSRV 1.74.1 (pinned via `rust-toolchain.toml`)
- Formatting rules in `.rustfmt.toml`; linter config in `clippy.toml`
- The `sample/` directory contains an Xcode project used by `mise ios` and CI
- CI runs on `macos-26` and requires `xcpretty` to be installed
- Releases are triggered by commits with messages containing `"Prepare version to"` — the workflow extracts the semver and publishes a GitHub Release using `CHANGELOG.md` content
