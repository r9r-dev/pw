# Task: pw CLI Password Generator

## Status: Done

## Description
Implement a fast, secure CLI password generator in Rust.

## Features implemented
- Random password generation (CSPRNG)
- Passphrase generation (EFF diceware wordlist)
- 8 predefined profiles (pin, pin6, wifi, strong, alpha, hex, memorable, uuid)
- Entropy analysis (zxcvbn)
- Clipboard support (arboard)
- Colored output with strength indicators
- Quiet mode for piping
- Character set customization (exclude uppercase, lowercase, digits, symbols, ambiguous)
- Custom symbol sets
- Multiple password generation

## Tests
- 26 unit tests
- 14 integration tests
- All passing, clippy clean, fmt clean
