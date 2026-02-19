use crate::generator::charset::CharsetBuilder;
use crate::generator::passphrase::{self, PassphraseOptions};
use crate::generator::random;

pub struct Profile {
    pub name: &'static str,
    pub description: &'static str,
    kind: ProfileKind,
}

enum ProfileKind {
    Charset {
        length: usize,
        builder: fn() -> CharsetBuilder,
    },
    Passphrase(PassphraseOptions),
    Uuid,
}

impl Profile {
    pub fn generate(&self) -> String {
        match &self.kind {
            ProfileKind::Charset { length, builder } => {
                let charset = builder().build();
                random::generate_password(&charset, *length)
            }
            ProfileKind::Passphrase(opts) => passphrase::generate_passphrase(opts),
            ProfileKind::Uuid => random::generate_uuid(),
        }
    }
}

pub fn get_profiles() -> Vec<Profile> {
    vec![
        Profile {
            name: "pin",
            description: "4-digit PIN",
            kind: ProfileKind::Charset {
                length: 4,
                builder: || {
                    CharsetBuilder::new()
                        .uppercase(false)
                        .lowercase(false)
                        .symbols(false)
                },
            },
        },
        Profile {
            name: "pin6",
            description: "6-digit PIN",
            kind: ProfileKind::Charset {
                length: 6,
                builder: || {
                    CharsetBuilder::new()
                        .uppercase(false)
                        .lowercase(false)
                        .symbols(false)
                },
            },
        },
        Profile {
            name: "wifi",
            description: "63 chars, all sets, no ambiguous",
            kind: ProfileKind::Charset {
                length: 63,
                builder: || CharsetBuilder::new().no_ambiguous(true),
            },
        },
        Profile {
            name: "strong",
            description: "32 chars, all character sets",
            kind: ProfileKind::Charset {
                length: 32,
                builder: CharsetBuilder::new,
            },
        },
        Profile {
            name: "alpha",
            description: "16 chars, letters only",
            kind: ProfileKind::Charset {
                length: 16,
                builder: || CharsetBuilder::new().digits(false).symbols(false),
            },
        },
        Profile {
            name: "hex",
            description: "32 hex chars",
            kind: ProfileKind::Charset {
                length: 32,
                builder: || CharsetBuilder::new().hex_only(true),
            },
        },
        Profile {
            name: "memorable",
            description: "Passphrase with 4 words",
            kind: ProfileKind::Passphrase(PassphraseOptions::default()),
        },
        Profile {
            name: "uuid",
            description: "UUID v4 format",
            kind: ProfileKind::Uuid,
        },
    ]
}

pub fn find_profile(name: &str) -> Option<Profile> {
    get_profiles().into_iter().find(|p| p.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_profile_is_4_digits() {
        let profile = find_profile("pin").unwrap();
        let pw = profile.generate();
        assert_eq!(pw.len(), 4);
        assert!(pw.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn pin6_profile_is_6_digits() {
        let profile = find_profile("pin6").unwrap();
        let pw = profile.generate();
        assert_eq!(pw.len(), 6);
        assert!(pw.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn hex_profile_is_32_hex() {
        let profile = find_profile("hex").unwrap();
        let pw = profile.generate();
        assert_eq!(pw.len(), 32);
        assert!(pw.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn uuid_profile_format() {
        let profile = find_profile("uuid").unwrap();
        let pw = profile.generate();
        assert_eq!(pw.len(), 36);
        assert_eq!(pw.matches('-').count(), 4);
    }

    #[test]
    fn all_profiles_exist() {
        let names = [
            "pin",
            "pin6",
            "wifi",
            "strong",
            "alpha",
            "hex",
            "memorable",
            "uuid",
        ];
        for name in names {
            assert!(find_profile(name).is_some(), "Profile '{name}' not found");
        }
    }

    #[test]
    fn unknown_profile_returns_none() {
        assert!(find_profile("nonexistent").is_none());
    }
}
