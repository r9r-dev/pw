mod cli;
mod clipboard;
mod entropy;
mod generator;
mod output;
mod profile;
mod wordlist;

use clap::Parser;
use cli::{Cli, Command};
use generator::charset::CharsetBuilder;
use generator::passphrase::{self, PassphraseOptions};
use generator::random;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Passphrase {
            words,
            separator,
            capitalize,
            append_digit,
            count,
            copy,
            entropy: show_entropy,
            quiet,
        }) => {
            let opts = PassphraseOptions {
                words,
                separator,
                capitalize,
                append_digit,
            };
            run_generate(count, copy, show_entropy, quiet, || {
                passphrase::generate_passphrase(&opts)
            });
        }
        Some(Command::Profile {
            name,
            copy,
            entropy: show_entropy,
            quiet,
            count,
        }) => {
            let prof = match profile::find_profile(&name) {
                Some(p) => p,
                None => {
                    eprintln!("Unknown profile: {name}");
                    eprintln!("Use 'pw profiles' to list available profiles.");
                    std::process::exit(1);
                }
            };
            run_generate(count, copy, show_entropy, quiet, || prof.generate());
        }
        Some(Command::Profiles) => {
            println!("Available profiles:");
            println!();
            for p in profile::get_profiles() {
                println!("  {:<12} {}", p.name, p.description);
            }
        }
        None => {
            let charset = CharsetBuilder::new()
                .uppercase(!cli.no_uppercase)
                .lowercase(!cli.no_lowercase)
                .digits(!cli.no_digits)
                .symbols(!cli.no_symbols)
                .no_ambiguous(cli.no_ambiguous)
                .custom_symbols(cli.custom_symbols.clone())
                .build();

            let length = cli.length;
            run_generate(cli.count, cli.copy, cli.entropy, cli.quiet, || {
                random::generate_password(&charset, length)
            });
        }
    }
}

fn run_generate(
    count: usize,
    copy: bool,
    show_entropy: bool,
    quiet: bool,
    generate: impl Fn() -> String,
) {
    let mut last = String::new();

    for i in 0..count {
        let password = generate();
        output::print_password(&password, show_entropy, quiet);
        if !quiet && count > 1 && i < count - 1 && show_entropy {
            println!();
        }
        last = password;
    }

    if copy {
        match clipboard::copy_to_clipboard(&last) {
            Ok(()) => output::print_copy_notice(quiet),
            Err(e) => eprintln!("  {e}"),
        }
    }
}
