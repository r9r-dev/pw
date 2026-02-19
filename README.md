# pw

A fast, secure CLI password generator built in Rust.

## Installation

### Homebrew (macOS/Linux)

```bash
brew install r9r-dev/tap/pw
```

### From source

```bash
cargo install --path .
```

## Quick usage

```bash
pw                          # 16-char password (default)
pw -l 32 -e                 # 32 chars + entropy info
pw -n 5 -q                  # 5 passwords, raw output
pw passphrase               # 4-word passphrase
pw passphrase -w 6          # 6-word passphrase
pw profile pin              # 4-digit PIN
pw profile wifi             # 63-char WiFi key
pw profiles                 # List all profiles
pw -c                       # Copy to clipboard
```

## Documentation

See [docs/USER_MANUAL.md](docs/USER_MANUAL.md) for full documentation.
