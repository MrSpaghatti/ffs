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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Command, Correction};

    #[derive(Debug)]
    struct MockRule {
        name: String,
        priority: usize,
    }

    impl MockRule {
        fn new(name: &str, priority: usize) -> Self {
            Self {
                name: name.to_string(),
                priority,
            }
        }
    }

    impl Rule for MockRule {
        fn name(&self) -> &str {
            &self.name
        }
        fn matches(&self, _command: &Command) -> bool {
            true
        }
        fn generate_corrections(&self, _command: &Command) -> Vec<Correction> {
            vec![Correction::new(format!("fixed_{}", self.name), false, self.priority)]
        }
    }

    #[test]
    fn test_register_rule_whitelist() {
        let mut config = Config::default();
        config.rules = Some(vec!["rule1".to_string()]);

        let mut engine = Engine::new(config);
        engine.register_rule(Arc::new(MockRule::new("rule1", 100)));
        engine.register_rule(Arc::new(MockRule::new("rule2", 100)));

        assert_eq!(engine.rules.len(), 1);
        assert_eq!(engine.rules[0].name(), "rule1");
    }

    #[test]
    fn test_register_rule_exclude() {
        let mut config = Config::default();
        config.exclude_rules = Some(vec!["rule2".to_string()]);

        let mut engine = Engine::new(config);
        engine.register_rule(Arc::new(MockRule::new("rule1", 100)));
        engine.register_rule(Arc::new(MockRule::new("rule2", 100)));

        assert_eq!(engine.rules.len(), 1);
        assert_eq!(engine.rules[0].name(), "rule1");
    }

    #[test]
    fn test_get_corrections_priority() {
        let config = Config::default();
        let mut engine = Engine::new(config);

        engine.register_rule(Arc::new(MockRule::new("low", 10)));
        engine.register_rule(Arc::new(MockRule::new("high", 100)));
        engine.register_rule(Arc::new(MockRule::new("medium", 50)));

        let command = Command::new("foo".to_string(), "".to_string(), "".to_string());
        let corrections = engine.get_corrections(&command);

        assert_eq!(corrections.len(), 3);
        assert_eq!(corrections[0].command, "fixed_high");
        assert_eq!(corrections[1].command, "fixed_medium");
        assert_eq!(corrections[2].command, "fixed_low");
    }
}
