use colored::*;

pub fn print_banner() {
    println!("\n");

    // ASCII Art Logo با رنگ‌های مختلف
    println!(
        "{}",
        "██╗    ██╗ ██████╗ ██████╗ ██████╗ ███████╗███╗   ██╗"
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        "██║    ██║██╔═══██╗██╔══██╗██╔══██╗██╔════╝████╗  ██║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "██║ █╗ ██║██║   ██║██████╔╝██████╔╝█████╗  ██╔██╗ ██║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "██║███╗██║██║   ██║██╔══██╗██╔═══╝ ██╔══╝  ██║╚██╗██║"
            .blue()
            .bold()
    );
    println!(
        "{}",
        "╚███╔███╔╝╚██████╔╝██║  ██║██║     ███████╗██║ ╚████║"
            .bright_magenta()
            .bold()
    );
    println!(
        "{}",
        " ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═══╝"
            .magenta()
            .bold()
    );

    println!(
        "\n{}",
        "    🔥 LOGIC-AS-DATA • ZERO-COST INLINING • BYTECODE VM 🔥"
            .bright_yellow()
            .bold()
    );
    println!("\n{}", "⚡ W O R P E N ⚡".bright_white().bold());
    println!(
        "{}\n",
        "ᴛʜᴇ ᴅʏɴᴀᴍɪᴄ ᴀᴘɪ ᴇɴɢɪɴᴇ • ᴘᴏᴡᴇʀᴇᴅ ʙʏ ʀᴜsᴛ 🦀".bright_green()
    );

    println!(
        "{}",
        "═══════════════════════════════════════════════════════".bright_black()
    );
    println!(
        "{} {}",
        "Version:".bright_white(),
        env!("CARGO_PKG_VERSION").green()
    );
    println!("{} {}", "Author:".bright_white(), "Matin Sanei".cyan());
    println!(
        "{}",
        "═══════════════════════════════════════════════════════".bright_black()
    );
    println!("\n");
}
