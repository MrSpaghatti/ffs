use dialoguer::{MultiSelect, Select, theme::ColorfulTheme};
use crate::types::Correction;
use colored::*;

pub fn select_correction(corrections: &[Correction], select_multiple: bool) -> Vec<&Correction> {
    if corrections.is_empty() {
        return vec![];
    }

    let options: Vec<String> = corrections.iter()
        .map(|c| format!("{} {}", c.command.bold(), if c.side_effect { "(side effect)" } else { "" }))
        .collect();

    if select_multiple && corrections.len() > 1 {
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select multiple corrections (Space to select, Enter to confirm):")
            .items(&options)
            .interact()
            .unwrap_or_default();

        selections.into_iter().map(|idx| &corrections[idx]).collect()
    } else {
        let mut options_display = options.clone();
        options_display.push("Cancel".red().to_string());

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a correction:")
            .default(0)
            .items(&options_display)
            .interact()
            .unwrap_or(options.len()); // Default to cancel on error

        if selection < corrections.len() {
            vec![&corrections[selection]]
        } else {
            vec![]
        }
    }
}
