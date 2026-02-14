use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Create a styled spinner for loading operations
pub fn create_spinner(msg: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("  {spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"]),
    );
    spinner.set_message(msg.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}
