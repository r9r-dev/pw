use colored::Colorize;

pub struct EntropyInfo {
    pub bits: f64,
    pub score: u8,
    pub crack_time: String,
}

#[allow(dead_code)]
pub fn calculate_entropy(charset_size: usize, length: usize) -> f64 {
    (charset_size as f64).log2() * length as f64
}

#[allow(dead_code)]
pub fn calculate_passphrase_entropy(
    wordlist_size: usize,
    word_count: usize,
    append_digit: bool,
) -> f64 {
    let mut bits = (wordlist_size as f64).log2() * word_count as f64;
    if append_digit {
        bits += 10f64.log2(); // ~3.32 bits
    }
    bits
}

pub fn analyze_password(password: &str) -> EntropyInfo {
    let estimate = zxcvbn::zxcvbn(password, &[]);
    let score = estimate.score().into();
    let crack_time = format_crack_time(&estimate);
    let bits = estimate.guesses_log10() * std::f64::consts::LOG2_10; // convert log10(guesses) to bits

    EntropyInfo {
        bits,
        score,
        crack_time,
    }
}

fn format_crack_time(estimate: &zxcvbn::Entropy) -> String {
    let seconds = estimate.crack_times().offline_slow_hashing_1e4_per_second();
    format!("{seconds}")
}

pub fn format_entropy_display(info: &EntropyInfo) -> String {
    let bar = strength_bar(info.score);
    let label = strength_label(info.score);
    format!(
        "  Entropy: {:.1} bits | Strength: {} {} | Crack time: {}",
        info.bits, bar, label, info.crack_time
    )
}

fn strength_bar(score: u8) -> String {
    let filled = score as usize + 1;
    let empty = 4_usize.saturating_sub(score as usize);
    let bar_filled: String = (0..filled).map(|_| '\u{2588}').collect(); // Full block
    let bar_empty: String = (0..empty).map(|_| '\u{2591}').collect(); // Light shade

    match score {
        0 => format!("{}{}", bar_filled.red(), bar_empty.dimmed()),
        1 => format!("{}{}", bar_filled.red(), bar_empty.dimmed()),
        2 => format!("{}{}", bar_filled.yellow(), bar_empty.dimmed()),
        3 => format!("{}{}", bar_filled.green(), bar_empty.dimmed()),
        4 => format!("{}{}", bar_filled.bright_green(), bar_empty.dimmed()),
        _ => format!("{}{}", bar_filled, bar_empty),
    }
}

fn strength_label(score: u8) -> String {
    match score {
        0 => "Very weak".red().to_string(),
        1 => "Weak".red().to_string(),
        2 => "Fair".yellow().to_string(),
        3 => "Strong".green().to_string(),
        4 => "Very strong".bright_green().to_string(),
        _ => "Unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entropy_calculation() {
        // 95 printable ASCII chars, 16 length = ~105 bits
        let bits = calculate_entropy(95, 16);
        assert!((bits - 105.0).abs() < 1.0);
    }

    #[test]
    fn passphrase_entropy_calculation() {
        // 7776 words, 4 words = ~51.7 bits
        let bits = calculate_passphrase_entropy(7776, 4, false);
        assert!((bits - 51.7).abs() < 0.1);
    }

    #[test]
    fn passphrase_entropy_with_digit() {
        let without = calculate_passphrase_entropy(7776, 4, false);
        let with = calculate_passphrase_entropy(7776, 4, true);
        assert!(with > without);
        assert!((with - without - 10f64.log2()).abs() < 0.01);
    }

    #[test]
    fn analyze_weak_password() {
        let info = analyze_password("1234");
        assert!(info.score <= 1);
    }

    #[test]
    fn analyze_strong_password() {
        let info = analyze_password("j8#kL!m9@nQ2$pR5");
        assert!(info.score >= 3);
    }
}
