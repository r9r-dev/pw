use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pw", version, about = "A fast, secure password generator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Password length
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    /// Number of passwords to generate
    #[arg(short = 'n', long = "count", default_value_t = 1)]
    pub count: usize,

    /// Copy to clipboard
    #[arg(short, long)]
    pub copy: bool,

    /// Show entropy and strength
    #[arg(short, long)]
    pub entropy: bool,

    /// Quiet output (raw, for piping)
    #[arg(short, long)]
    pub quiet: bool,

    /// Exclude uppercase letters
    #[arg(short = 'U', long = "no-uppercase")]
    pub no_uppercase: bool,

    /// Exclude lowercase letters
    #[arg(short = 'L', long = "no-lowercase")]
    pub no_lowercase: bool,

    /// Exclude digits
    #[arg(short = 'D', long = "no-digits")]
    pub no_digits: bool,

    /// Exclude symbols
    #[arg(short = 'S', long = "no-symbols")]
    pub no_symbols: bool,

    /// Exclude ambiguous characters (0O, 1lI...)
    #[arg(short = 'A', long = "no-ambiguous")]
    pub no_ambiguous: bool,

    /// Custom symbol set
    #[arg(long = "symbols")]
    pub custom_symbols: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate a passphrase (words)
    Passphrase {
        /// Number of words
        #[arg(short, long, default_value_t = 4)]
        words: usize,

        /// Word separator
        #[arg(short, long, default_value = "-")]
        separator: String,

        /// Capitalize each word
        #[arg(long)]
        capitalize: bool,

        /// Append a digit at the end
        #[arg(long)]
        append_digit: bool,

        /// Number of passphrases to generate
        #[arg(short = 'n', long = "count", default_value_t = 1)]
        count: usize,

        /// Copy to clipboard
        #[arg(short, long)]
        copy: bool,

        /// Show entropy and strength
        #[arg(short, long)]
        entropy: bool,

        /// Quiet output (raw, for piping)
        #[arg(short, long)]
        quiet: bool,
    },
    /// Use a predefined profile
    Profile {
        /// Profile name
        name: String,

        /// Copy to clipboard
        #[arg(short, long)]
        copy: bool,

        /// Show entropy and strength
        #[arg(short, long)]
        entropy: bool,

        /// Quiet output (raw, for piping)
        #[arg(short, long)]
        quiet: bool,

        /// Number of passwords to generate
        #[arg(short = 'n', long = "count", default_value_t = 1)]
        count: usize,
    },
    /// List available profiles
    Profiles,
}
