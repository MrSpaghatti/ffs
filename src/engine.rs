use crate::rules::Rule;
use crate::types::{Command, Correction};
use crate::config::Config;
use std::sync::Arc;

pub struct Engine {
    rules: Vec<Arc<dyn Rule>>,
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        Self {
            rules: Vec::new(),
            config,
        }
    }

    pub fn register_rule(&mut self, rule: Arc<dyn Rule>) {
        // Check exclusion/inclusion based on config
        let name = rule.name();
        if let Some(excluded) = &self.config.exclude_rules {
            if excluded.contains(&name.to_string()) {
                return;
            }
        }
        if let Some(included) = &self.config.rules {
            if !included.contains(&name.to_string()) {
                return; // Only include allowed rules if whitelist exists
            }
        }
        self.rules.push(rule);
    }

    pub fn get_corrections(&self, command: &Command) -> Vec<Correction> {
        let mut corrections = Vec::new();

        for rule in &self.rules {
            if rule.matches(command) {
                let mut new_corrections = rule.generate_corrections(command);
                corrections.append(&mut new_corrections);
            }
        }

        // Sort by priority
        corrections.sort_by(|a, b| b.priority.cmp(&a.priority));

        corrections
    }
}
