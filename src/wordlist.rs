use std::sync::LazyLock;

static WORDLIST: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let raw = include_str!("../data/eff_large_wordlist.txt");
    raw.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            // Format: "11111\tword"
            line.split('\t').nth(1)
        })
        .collect()
});

pub fn get_wordlist() -> &'static [&'static str] {
    &WORDLIST
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wordlist_loaded() {
        let wl = get_wordlist();
        assert_eq!(wl.len(), 7776);
    }

    #[test]
    fn words_are_valid() {
        let wl = get_wordlist();
        for word in wl {
            assert!(
                word.chars().all(|c| c.is_alphabetic() || c == '-'),
                "Invalid word: {word}"
            );
        }
    }
}
