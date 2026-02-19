use assert_cmd::Command;
use predicates::prelude::*;

fn pw() -> Command {
    Command::cargo_bin("pw").unwrap()
}

#[test]
fn default_generates_16_chars() {
    pw().arg("-q")
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| s.trim().len() == 16));
}

#[test]
fn custom_length() {
    pw().args(["-l", "32", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| s.trim().len() == 32));
}

#[test]
fn multiple_passwords() {
    pw().args(["-n", "5", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| s.trim().lines().count() == 5));
}

#[test]
fn digits_only() {
    pw().args(["-U", "-L", "-S", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| {
            s.trim().chars().all(|c| c.is_ascii_digit())
        }));
}

#[test]
fn show_entropy() {
    pw().arg("-e")
        .assert()
        .success()
        .stdout(predicate::str::contains("Entropy:"));
}

#[test]
fn passphrase_default() {
    pw().args(["passphrase", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| {
            s.trim().split('-').count() == 4
        }));
}

#[test]
fn passphrase_custom_words() {
    pw().args(["passphrase", "-w", "6", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| {
            s.trim().split('-').count() == 6
        }));
}

#[test]
fn passphrase_capitalize() {
    pw().args(["passphrase", "--capitalize", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| {
            s.trim()
                .split('-')
                .all(|w| w.chars().next().unwrap().is_uppercase())
        }));
}

#[test]
fn profile_pin() {
    pw().args(["profile", "pin", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| {
            let pw = s.trim();
            pw.len() == 4 && pw.chars().all(|c| c.is_ascii_digit())
        }));
}

#[test]
fn profile_hex() {
    pw().args(["profile", "hex", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| {
            let pw = s.trim();
            pw.len() == 32 && pw.chars().all(|c| c.is_ascii_hexdigit())
        }));
}

#[test]
fn profile_uuid() {
    pw().args(["profile", "uuid", "-q"])
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| {
            let pw = s.trim();
            pw.len() == 36 && pw.matches('-').count() == 4
        }));
}

#[test]
fn profiles_list() {
    pw().arg("profiles")
        .assert()
        .success()
        .stdout(predicate::str::contains("pin"))
        .stdout(predicate::str::contains("wifi"))
        .stdout(predicate::str::contains("strong"))
        .stdout(predicate::str::contains("uuid"));
}

#[test]
fn unknown_profile_fails() {
    pw().args(["profile", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown profile"));
}

#[test]
fn help_flag() {
    pw().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("password generator"));
}
