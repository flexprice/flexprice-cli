use colored::Colorize;
use tabled::{Table, settings::{Style, themes::Colorization, Color}};
use tabled::settings::object::Rows;

/// Format data as a pretty table or JSON based on output preference
pub fn print_table<T: tabled::Tabled>(items: &[T], output_json: bool) -> String
where
    T: serde::Serialize,
{
    if output_json {
        serde_json::to_string_pretty(items).unwrap_or_else(|_| "[]".to_string())
    } else if items.is_empty() {
        format!("  {}", "No results found.".dimmed())
    } else {
        let mut table = Table::new(items);
        table.with(Style::rounded());
        table.with(Colorization::exact([Color::new("\x1b[1;36m", "\x1b[0m")], Rows::first()));
        table.to_string()
    }
}

/// Print a single item as pretty JSON or a key-value display
pub fn print_detail<T: serde::Serialize>(item: &T, output_json: bool) -> String {
    if output_json {
        serde_json::to_string_pretty(item).unwrap_or_else(|_| "{}".to_string())
    } else {
        // Use colored JSON for non-json output too (looks nice)
        let json = serde_json::to_string_pretty(item).unwrap_or_else(|_| "{}".to_string());
        colorize_json(&json)
    }
}

/// Colorize a JSON string for terminal output
fn colorize_json(json: &str) -> String {
    let mut result = String::new();
    for line in json.lines() {
        let colored_line = if line.contains(':') {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                format!("{}: {}", parts[0].cyan(), parts[1])
            } else {
                line.to_string()
            }
        } else {
            line.dimmed().to_string()
        };
        result.push_str(&colored_line);
        result.push('\n');
    }
    result
}

/// Print a success message with a checkmark
pub fn success(msg: &str) {
    println!("  {} {}", "✓".green().bold(), msg);
}

/// Print an error message
pub fn error(msg: &str) {
    eprintln!("  {} {}", "✗".red().bold(), msg);
}

/// Print a warning message
pub fn warning(msg: &str) {
    eprintln!("  {} {}", "⚠".yellow().bold(), msg);
}

/// Print an info message
pub fn info(msg: &str) {
    println!("  {} {}", "ℹ".blue().bold(), msg);
}

/// Status badge with color based on status string
pub fn status_badge(status: &str) -> String {
    match status.to_lowercase().as_str() {
        "active" | "published" | "paid" | "finalized" => format!("{}", status.green().bold()),
        "draft" | "pending" => format!("{}", status.yellow()),
        "cancelled" | "canceled" | "void" | "voided" | "inactive" => format!("{}", status.red()),
        "trialing" | "paused" => format!("{}", status.blue()),
        _ => status.to_string(),
    }
}

/// The FlexPrice ASCII art banner
pub fn print_banner() {
    let banner = r#"
    ╔═══════════════════════════════════════════╗
    ║                                           ║
    ║   ⚡ FlexPrice CLI                        ║
    ║   Usage-based billing, made simple.       ║
    ║                                           ║
    ╚═══════════════════════════════════════════╝
    "#;
    println!("{}", banner.cyan());
}
