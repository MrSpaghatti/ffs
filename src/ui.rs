use dialoguer::{Select, theme::ColorfulTheme};
use crate::types::Correction;
use colored::*;

pub fn select_correction(corrections: &[Correction]) -> Option<&Correction> {
    if corrections.is_empty() {
        return None;
    }

    // If only one, `thefuck` usually just gives it (or requires confirmation).
    // Here we show a UI.

    let options: Vec<String> = corrections.iter()
        .map(|c| format!("{} {}", c.command.bold(), if c.side_effect { "(side effect)" } else { "" }))
        .collect();

    // Add "Cancel"
    let mut options_display = options.clone();
    options_display.push("Cancel".red().to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a correction:")
        .default(0)
        .items(&options_display)
        .interact()
        .unwrap_or(options.len()); // Default to cancel on error

    if selection < corrections.len() {
        Some(&corrections[selection])
    } else {
        None
    }
}
