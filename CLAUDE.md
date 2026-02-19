# pw - CLI Password Generator

## Overview
Fast, secure CLI password generator in Rust using CSPRNG (ChaCha12).

## Build & Test
```bash
cargo build            # Build
cargo test             # Run all tests (26 unit + 14 integration)
cargo clippy           # Lint
cargo fmt              # Format
```

## Project Structure
```
src/
  main.rs              # Entry point, CLI dispatch
  cli.rs               # Clap derive structs
  generator/
    charset.rs         # Charset builder
    random.rs          # CSPRNG generation + UUID
    passphrase.rs      # Diceware passphrase
  profile.rs           # Predefined profiles
  entropy.rs           # zxcvbn analysis
  clipboard.rs         # Clipboard (arboard)
  output.rs            # Colored output
  wordlist.rs          # EFF wordlist (include_str!)
data/
  eff_large_wordlist.txt  # 7776 EFF diceware words
tests/
  integration.rs       # CLI tests (assert_cmd)
```

## Dependencies
clap 4 (derive), rand 0.9, arboard 3, zxcvbn 3, colored 3

## Key notes
- Rust edition 2024: `gen` is a reserved keyword
- Wordlist contains hyphenated words (e.g. "drop-down")
- CI: `.github/workflows/ci.yml` (fmt, clippy, tests)
- Release: `.github/workflows/release.yml` (cross-platform + Homebrew tap)
- Distribution: `r9r-dev/homebrew-pw` tap repo (needs `TAP_TOKEN` secret)

## Profiles
pin, pin6, wifi, strong, alpha, hex, memorable, uuid
