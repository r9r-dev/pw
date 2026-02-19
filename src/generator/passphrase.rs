use rand::Rng;

use crate::wordlist;

pub struct PassphraseOptions {
    pub words: usize,
    pub separator: String,
    pub capitalize: bool,
    pub append_digit: bool,
}

impl Default for PassphraseOptions {
    fn default() -> Self {
        Self {
            words: 4,
            separator: "-".to_string(),
            capitalize: false,
            append_digit: false,
        }
    }
}

pub fn generate_passphrase(opts: &PassphraseOptions) -> String {
    let wordlist = wordlist::get_wordlist();
    let mut rng = rand::rng();

    let selected: Vec<String> = (0..opts.words)
        .map(|_| {
            let idx = rng.random_range(0..wordlist.len());
            let word = wordlist[idx].to_string();
            if opts.capitalize {
                let mut chars = word.chars();
                match chars.next() {
                    Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                    None => word,
                }
            } else {
                word
            }
        })
        .collect();

    let mut result = selected.join(&opts.separator);

    if opts.append_digit {
        let digit = rng.random_range(0..10u8);
        result.push_str(&digit.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_passphrase_has_4_words() {
        let opts = PassphraseOptions::default();
        let pp = generate_passphrase(&opts);
        let word_count = pp.split('-').count();
        assert_eq!(word_count, 4);
    }

    #[test]
    fn custom_separator() {
        let opts = PassphraseOptions {
            separator: ".".to_string(),
            ..Default::default()
        };
        let pp = generate_passphrase(&opts);
        assert!(pp.contains('.'));
        assert!(!pp.contains('-'));
    }

    #[test]
    fn capitalize_words() {
        let opts = PassphraseOptions {
            capitalize: true,
            ..Default::default()
        };
        let pp = generate_passphrase(&opts);
        for word in pp.split('-') {
            assert!(word.chars().next().unwrap().is_uppercase());
        }
    }

    #[test]
    fn append_digit() {
        let opts = PassphraseOptions {
            append_digit: true,
            ..Default::default()
        };
        let pp = generate_passphrase(&opts);
        assert!(pp.chars().last().unwrap().is_ascii_digit());
    }
}
