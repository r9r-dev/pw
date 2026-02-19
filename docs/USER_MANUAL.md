# pw - User Manual

## Overview

`pw` is a fast, secure CLI password generator. It uses CSPRNG (ChaCha12) for cryptographically secure random generation.

## Commands

### Default: generate a password

```bash
pw [OPTIONS]
```

Generates a random password (default: 16 characters, all character sets).

### Passphrase

```bash
pw passphrase [OPTIONS]
```

Generates a passphrase using the EFF diceware wordlist (7776 words).

### Profile

```bash
pw profile <NAME>
```

Generates a password using a predefined profile.

### List profiles

```bash
pw profiles
```

## Options

### Password options

| Option | Description | Default |
|--------|-------------|---------|
| `-l, --length <N>` | Password length | 16 |
| `-n, --count <N>` | Number of passwords | 1 |
| `-c, --copy` | Copy to clipboard | - |
| `-e, --entropy` | Show entropy and strength | - |
| `-q, --quiet` | Raw output (for piping) | - |
| `-U, --no-uppercase` | Exclude uppercase | - |
| `-L, --no-lowercase` | Exclude lowercase | - |
| `-D, --no-digits` | Exclude digits | - |
| `-S, --no-symbols` | Exclude symbols | - |
| `-A, --no-ambiguous` | Exclude ambiguous chars (0O, 1lI...) | - |
| `--symbols <CHARS>` | Custom symbol set | - |

### Passphrase options

| Option | Description | Default |
|--------|-------------|---------|
| `-w, --words <N>` | Number of words | 4 |
| `-s, --separator <C>` | Word separator | `-` |
| `--capitalize` | Capitalize each word | - |
| `--append-digit` | Append a digit at the end | - |

## Profiles

| Name | Description |
|------|-------------|
| `pin` | 4-digit PIN |
| `pin6` | 6-digit PIN |
| `wifi` | 63 chars, all sets, no ambiguous |
| `strong` | 32 chars, all character sets |
| `alpha` | 16 chars, letters only |
| `hex` | 32 hex chars |
| `memorable` | Passphrase with 4 words |
| `uuid` | UUID v4 format |

## Entropy display

Use `-e` to show password strength analysis:
- Theoretical entropy (bits)
- zxcvbn strength score (0-4)
- Estimated crack time

## Piping and scripting

Use `-q` for raw output suitable for piping:

```bash
pw -q | pbcopy                    # Copy on macOS
pw -n 10 -q > passwords.txt      # Save to file
pw -q -l 32 | xargs echo -n      # No trailing newline
```

## Notes

- **Clipboard on Linux (Wayland)**: clipboard content is lost when the process terminates. This is inherent to the Wayland protocol.
- **CSPRNG**: uses `ThreadRng` (ChaCha12) with automatic re-seeding from the OS entropy source.
