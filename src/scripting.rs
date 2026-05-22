/*!
# Rhai Scripting API for FFS

Rules written in Rhai can implement the following functions:
- `fn matches() -> bool`: Determines if the rule applies to the command.
- `fn get_new_command() -> String | CorrectionBuilder`: Returns the corrected command or a builder object.

Available functions:
- `which(cmd: String) -> bool`: Checks if a command exists in PATH.
- `is_match(pattern: String, text: String) -> bool`: Checks if text matches the regex pattern.
- `replace(pattern: String, text: String, replacement: String) -> String`: Replaces matches in text using regex pattern.
- `side_effect(command: String) -> CorrectionBuilder`: Creates a correction with side_effect set to true.

A Rule can optionally declare `let priority = <number>;` at the top level to set its priority.
*/
use rhai::{Engine, Scope, AST, Dynamic, CustomType, TypeBuilder};
use crate::types::{Command, Correction};
use crate::rules::Rule;
use std::sync::Arc;
use std::fmt;
use std::path::Path;
use std::fs;
use regex::Regex;

#[derive(Clone, CustomType)]
pub struct CorrectionBuilder {
    pub command: String,
    pub side_effect: bool,
    pub priority: usize,
}

impl CorrectionBuilder {
    pub fn new(command: String) -> Self {
        Self {
            command,
            side_effect: false,
            priority: 100,
        }
    }
}
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
    pub fn new(name: String, script: &str, default_priority: usize) -> Self {
        let mut engine = Engine::new();

        engine.build_type::<CorrectionBuilder>();

        engine.register_fn("side_effect", |cmd: String| {
            let mut builder = CorrectionBuilder::new(cmd);
            builder.side_effect = true;
            builder
        });

        engine.register_fn("which", |cmd: String| {
            which::which(cmd).is_ok()
        });

        engine.register_fn("is_match", |pattern: String, text: String| {
            if let Ok(re) = Regex::new(&pattern) {
                re.is_match(&text)
            } else {
                false
            }
        });

        engine.register_fn("replace", |pattern: String, text: String, repl: String| {
            if let Ok(re) = Regex::new(&pattern) {
                re.replace_all(&text, repl.as_str()).into_owned()
            } else {
                text
            }
        });

        let ast = engine.compile(script).expect("Failed to compile rhai script");

        let mut scope = Scope::new();
        let mut priority = default_priority;
        if let Ok(_) = engine.eval_ast_with_scope::<()>(&mut scope, &ast) {
            if let Some(p) = scope.get_value::<i64>("priority") {
                priority = p as usize;
            }
        }

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

        let result: Dynamic = match self.engine.call_fn(&mut scope, &self.ast, "get_new_command", ()) {
            Ok(res) => res,
            Err(_) => return vec![],
        };

        if let Some(cmd_str) = result.clone().try_cast::<String>() {
            vec![Correction::new(cmd_str, false, self.priority)]
        } else if let Some(builder) = result.try_cast::<CorrectionBuilder>() {
            let p = if builder.priority != 100 { builder.priority } else { self.priority };
            vec![Correction::new(builder.command, builder.side_effect, p)]
        } else {
            vec![]
        }
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
