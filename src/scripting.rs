use rhai::{Engine, Scope, AST};
use crate::types::{Command, Correction};
use crate::rules::Rule;
use std::sync::Arc;
use std::fmt;
use std::path::Path;
use std::fs;

#[derive(Clone)]
pub struct RhaiRule {
    engine: Arc<Engine>,
    ast: AST,
    name: String,
    priority: usize,
}

impl fmt::Debug for RhaiRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RhaiRule")
            .field("name", &self.name)
            .field("priority", &self.priority)
            .finish()
    }
}

impl RhaiRule {
    pub fn new(name: String, script: &str, priority: usize) -> Self {
        let engine = Engine::new();
        // In a real app we might want to handle compilation errors gracefully, but for now we expect valid scripts
        let ast = engine.compile(script).expect("Failed to compile rhai script");
        Self {
            engine: Arc::new(engine),
            ast,
            name,
            priority,
        }
    }
}

impl Rule for RhaiRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn matches(&self, command: &Command) -> bool {
        let mut scope = Scope::new();
        scope.push("script", command.script.clone());
        scope.push("stdout", command.stdout.clone());
        scope.push("stderr", command.stderr.clone());

        let result: bool = self.engine.call_fn(&mut scope, &self.ast, "matches", ()).unwrap_or(false);
        result
    }

    fn generate_corrections(&self, command: &Command) -> Vec<Correction> {
        let mut scope = Scope::new();
        scope.push("script", command.script.clone());
        scope.push("stdout", command.stdout.clone());
        scope.push("stderr", command.stderr.clone());

        let result: String = match self.engine.call_fn(&mut scope, &self.ast, "get_new_command", ()) {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        vec![Correction::new(result, false, self.priority)]
    }
}

pub fn load_rhai_rules(dir: &Path) -> Vec<RhaiRule> {
    let mut rules = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "rhai") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let name = path.file_stem().unwrap().to_string_lossy().to_string();
                        // Priority default to 100 for now
                        rules.push(RhaiRule::new(name, &content, 100));
                    }
                }
            }
        }
    }

    rules
}
