#[derive(Debug, Clone)]
pub struct Command {
    pub script: String,
    pub stdout: String,
    pub stderr: String,
}

impl Command {
    pub fn new(script: String, stdout: String, stderr: String) -> Self {
        Self {
            script,
            stdout,
            stderr,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Correction {
    pub command: String,
    pub side_effect: bool,
    pub priority: usize,
    pub confirmation_text: Option<String>,
}

impl Correction {
    pub fn new(command: String, side_effect: bool, priority: usize) -> Self {
        Self {
            command,
            side_effect,
            priority,
            confirmation_text: None,
        }
    }
}
