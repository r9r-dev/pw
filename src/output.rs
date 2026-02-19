use colored::Colorize;

use crate::entropy;

pub fn print_password(password: &str, show_entropy: bool, quiet: bool) {
    if quiet {
        println!("{password}");
        return;
    }

    println!("  {}", password.bold());

    if show_entropy {
        let info = entropy::analyze_password(password);
        println!("{}", entropy::format_entropy_display(&info));
    }
}

pub fn print_copy_notice(quiet: bool) {
    if !quiet {
        println!("{}", "  Copied to clipboard.".dimmed());
    }
}
