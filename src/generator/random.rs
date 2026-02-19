use rand::Rng;

pub fn generate_password(charset: &[char], length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx]
        })
        .collect()
}

pub fn generate_uuid() -> String {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 16];
    for b in &mut bytes {
        *b = rng.random();
    }
    // Set version 4
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    // Set variant 1
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_has_correct_length() {
        let charset: Vec<char> = "abc123".chars().collect();
        let pw = generate_password(&charset, 20);
        assert_eq!(pw.len(), 20);
    }

    #[test]
    fn password_uses_only_charset() {
        let charset: Vec<char> = "abc".chars().collect();
        let pw = generate_password(&charset, 100);
        assert!(pw.chars().all(|c| charset.contains(&c)));
    }

    #[test]
    fn uuid_format() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36);
        let parts: Vec<&str> = uuid.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
        // Version 4
        assert!(parts[2].starts_with('4'));
    }
}
