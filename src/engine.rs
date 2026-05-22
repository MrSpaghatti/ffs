use crate::rules::Rule;
use crate::types::{Command, Correction};
use crate::config::Config;
use std::sync::Arc;
use rayon::prelude::*;
use std::time::Instant;

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

    pub fn register_rule(&mut self, mut rule: Box<dyn Rule>) {
        // Check exclusion/inclusion based on config
        let name = rule.name().to_string();
        if let Some(excluded) = &self.config.exclude_rules {
            if excluded.contains(&name) {
                return;
            }
        }
        if let Some(included) = &self.config.rules {
            if !included.contains(&name) {
                return; // Only include allowed rules if whitelist exists
            }
        }
        
        // Pass per-rule settings to the rule
        if let Some(rule_settings) = &self.config.rule_settings {
            if let Some(settings) = rule_settings.get(&name) {
                rule.init(settings);
            }
        }
        
        self.rules.push(Arc::from(rule));
    }

    pub fn get_corrections(&self, command: &Command) -> Vec<Correction> {
        let timeout_ms = self.config.slow_rule_timeout_ms.unwrap_or(500) as u128;

        let evaluate_rule = |rule: &Arc<dyn Rule>| -> Vec<Correction> {
            let start = Instant::now();
            let is_match = rule.matches(command);
            let elapsed = start.elapsed().as_millis();

            if elapsed > timeout_ms {
                eprintln!("Warning: Rule '{}' took {}ms to match, exceeding timeout of {}ms. Skipping.", rule.name(), elapsed, timeout_ms);
                return vec![];
            }

            if is_match {
                rule.generate_corrections(command)
            } else {
                vec![]
            }
        };

        let mut corrections: Vec<Correction> = if self.rules.len() > 20 {
            self.rules.par_iter()
                .flat_map(evaluate_rule)
                .collect()
        } else {
            self.rules.iter()
                .flat_map(evaluate_rule)
                .collect()
        };

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
        engine.register_rule(Box::new(MockRule::new("rule1", 100)));
        engine.register_rule(Box::new(MockRule::new("rule2", 100)));

        assert_eq!(engine.rules.len(), 1);
        assert_eq!(engine.rules[0].name(), "rule1");
    }

    #[test]
    fn test_register_rule_exclude() {
        let mut config = Config::default();
        config.exclude_rules = Some(vec!["rule2".to_string()]);

        let mut engine = Engine::new(config);
        engine.register_rule(Box::new(MockRule::new("rule1", 100)));
        engine.register_rule(Box::new(MockRule::new("rule2", 100)));

        assert_eq!(engine.rules.len(), 1);
        assert_eq!(engine.rules[0].name(), "rule1");
    }

    #[test]
    fn test_parallel_vs_sequential() {
        let config = Config::default();
        let mut engine_seq = Engine::new(config.clone());
        let mut engine_par = Engine::new(config.clone());

        // Less than 20 rules -> sequential
        for i in 0..10 {
            engine_seq.register_rule(Box::new(MockRule::new(&format!("rule_{}", i), i)));
        }

        // More than 20 rules -> parallel
        for i in 0..25 {
            engine_par.register_rule(Box::new(MockRule::new(&format!("rule_{}", i), i)));
        }

        let command = Command::new("foo".to_string(), "".to_string(), "".to_string());

        let seq_corrections = engine_seq.get_corrections(&command);
        assert_eq!(seq_corrections.len(), 10);

        let par_corrections = engine_par.get_corrections(&command);
        assert_eq!(par_corrections.len(), 25);

        // Verify both are sorted by priority (descending)
        for i in 0..9 {
            assert!(seq_corrections[i].priority >= seq_corrections[i+1].priority);
        }
        for i in 0..24 {
            assert!(par_corrections[i].priority >= par_corrections[i+1].priority);
        }
    }

    #[test]
    fn test_get_corrections_priority() {
        let config = Config::default();
        let mut engine = Engine::new(config);

        engine.register_rule(Box::new(MockRule::new("low", 10)));
        engine.register_rule(Box::new(MockRule::new("high", 100)));
        engine.register_rule(Box::new(MockRule::new("medium", 50)));

        let command = Command::new("foo".to_string(), "".to_string(), "".to_string());
        let corrections = engine.get_corrections(&command);

        assert_eq!(corrections.len(), 3);
        assert_eq!(corrections[0].command, "fixed_high");
        assert_eq!(corrections[1].command, "fixed_medium");
        assert_eq!(corrections[2].command, "fixed_low");
    }
}
