# pw - Architecture

## Tech Stack
- Language: Rust (edition 2024)
- CLI: clap 4 (derive)
- RNG: rand 0.9 (ChaCha12 CSPRNG)
- Clipboard: arboard 3
- Strength analysis: zxcvbn 3
- Colors: colored 3
- Testing: assert_cmd 2, predicates 3

## Project Structure

```
src/
  main.rs              # Entry point, CLI dispatch
  cli.rs               # Clap derive structs (args, subcommands)
  generator/
    mod.rs             # Module re-exports
    charset.rs         # Charset builder with toggles
    random.rs          # CSPRNG password generation + UUID
    passphrase.rs      # Diceware passphrase generation
  profile.rs           # Predefined profiles (pin, wifi, strong...)
  entropy.rs           # Entropy calculation + zxcvbn scoring
  clipboard.rs         # Clipboard wrapper (arboard)
  output.rs            # Output formatting (colors, quiet mode)
  wordlist.rs          # EFF wordlist (embedded via include_str!)
data/
  eff_large_wordlist.txt  # 7776 words, EFF diceware
tests/
  integration.rs       # CLI integration tests (assert_cmd)
```

## Subcommands
- (default): random password generation
- `passphrase`: diceware passphrase generation
- `profile <name>`: predefined profile
- `profiles`: list profiles

## Profiles
pin, pin6, wifi, strong, alpha, hex, memorable, uuid

## CI/CD
- `.github/workflows/ci.yml`: fmt, clippy, tests on push/PR
- `.github/workflows/release.yml`: cross-platform build + GitHub Release + Homebrew tap update
