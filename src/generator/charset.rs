const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?/~`";
const AMBIGUOUS: &str = "0O1lI|";
const HEX_CHARS: &str = "0123456789abcdef";

pub struct CharsetBuilder {
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
    no_ambiguous: bool,
    custom_symbols: Option<String>,
    hex_only: bool,
}

impl CharsetBuilder {
    pub fn new() -> Self {
        Self {
            uppercase: true,
            lowercase: true,
            digits: true,
            symbols: true,
            no_ambiguous: false,
            custom_symbols: None,
            hex_only: false,
        }
    }

    pub fn uppercase(mut self, v: bool) -> Self {
        self.uppercase = v;
        self
    }

    pub fn lowercase(mut self, v: bool) -> Self {
        self.lowercase = v;
        self
    }

    pub fn digits(mut self, v: bool) -> Self {
        self.digits = v;
        self
    }

    pub fn symbols(mut self, v: bool) -> Self {
        self.symbols = v;
        self
    }

    pub fn no_ambiguous(mut self, v: bool) -> Self {
        self.no_ambiguous = v;
        self
    }

    pub fn custom_symbols(mut self, s: Option<String>) -> Self {
        self.custom_symbols = s;
        self
    }

    pub fn hex_only(mut self, v: bool) -> Self {
        self.hex_only = v;
        self
    }

    pub fn build(self) -> Vec<char> {
        if self.hex_only {
            return HEX_CHARS.chars().collect();
        }

        let mut chars = String::new();

        if self.uppercase {
            chars.push_str(UPPERCASE);
        }
        if self.lowercase {
            chars.push_str(LOWERCASE);
        }
        if self.digits {
            chars.push_str(DIGITS);
        }
        if self.symbols {
            if let Some(ref custom) = self.custom_symbols {
                chars.push_str(custom);
            } else {
                chars.push_str(SYMBOLS);
            }
        }

        if self.no_ambiguous {
            chars.retain(|c| !AMBIGUOUS.contains(c));
        }

        let result: Vec<char> = chars.chars().collect();
        assert!(!result.is_empty(), "Charset cannot be empty");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_charset_has_all_types() {
        let chars = CharsetBuilder::new().build();
        assert!(chars.iter().any(|c| c.is_ascii_uppercase()));
        assert!(chars.iter().any(|c| c.is_ascii_lowercase()));
        assert!(chars.iter().any(|c| c.is_ascii_digit()));
        assert!(chars.iter().any(|c| !c.is_alphanumeric()));
    }

    #[test]
    fn no_ambiguous_removes_chars() {
        let chars = CharsetBuilder::new().no_ambiguous(true).build();
        assert!(!chars.contains(&'0'));
        assert!(!chars.contains(&'O'));
        assert!(!chars.contains(&'l'));
        assert!(!chars.contains(&'I'));
    }

    #[test]
    fn digits_only() {
        let chars = CharsetBuilder::new()
            .uppercase(false)
            .lowercase(false)
            .symbols(false)
            .build();
        assert!(chars.iter().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn hex_charset() {
        let chars = CharsetBuilder::new().hex_only(true).build();
        assert_eq!(chars.len(), 16);
        assert!(chars.iter().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn custom_symbols() {
        let chars = CharsetBuilder::new()
            .uppercase(false)
            .lowercase(false)
            .digits(false)
            .custom_symbols(Some("!@#".to_string()))
            .build();
        assert_eq!(chars, vec!['!', '@', '#']);
    }

    #[test]
    #[should_panic(expected = "Charset cannot be empty")]
    fn empty_charset_panics() {
        CharsetBuilder::new()
            .uppercase(false)
            .lowercase(false)
            .digits(false)
            .symbols(false)
            .build();
    }
}
