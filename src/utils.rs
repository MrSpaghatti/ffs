use std::env;

pub fn get_last_command() -> Option<String> {
    if let Ok(history) = env::var("TF_HISTORY") {
        let lines: Vec<&str> = history.trim().split('\n').collect();
        // The history is usually in order?
        // thefuck reverses it. "fc -ln -10" output order depends on shell but usually oldest first?
        // Wait, "fc -ln -10" in bash outputs the *last 10 commands*.
        // Typically they are in chronological order.
        // So the last one is the most recent.
        // But the *very* last one might be the `ffs` (alias) call itself if the shell adds it to history immediately?
        // Usually alias execution is added to history *after*.

        // Let's assume the last line is the most recent.
        // We iterate backwards.

        for line in lines.iter().rev() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("ffs") && !trimmed.starts_with("fuck") {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}
