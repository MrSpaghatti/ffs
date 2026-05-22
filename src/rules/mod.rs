use crate::types::{Command, Correction};
use std::fmt::Debug;

pub trait Rule: Send + Sync + Debug {
    fn matches(&self, command: &Command) -> bool;
    fn generate_corrections(&self, command: &Command) -> Vec<Correction>;
    fn name(&self) -> &str;

    // Optional initialization with per-rule settings
    #[allow(unused_variables)]
    fn init(&mut self, settings: &std::collections::HashMap<String, toml::Value>) {}
}

// Submodules for builtin rules
pub mod cargo;
pub mod git;
pub mod generic;
pub mod mkdir;
pub mod sudo;
pub mod cd;
pub mod python;
pub mod python_cmd;
pub mod grep;
pub mod rm;
pub mod apt_get;
pub mod brew;
pub mod cp;
pub mod ls;
pub mod git_add;

#[cfg(test)]
mod mkdir_tests;
#[cfg(test)]
mod sudo_tests;
#[cfg(test)]
mod cd_tests;
#[cfg(test)]
mod git_tests;
#[cfg(test)]
mod python_tests;
#[cfg(test)]
mod python_cmd_tests;
#[cfg(test)]
mod grep_tests;
#[cfg(test)]
mod rm_tests;
#[cfg(test)]
mod apt_get_tests;
#[cfg(test)]
mod brew_tests;
#[cfg(test)]
mod cp_tests;
#[cfg(test)]
mod ls_tests;
#[cfg(test)]
mod git_add_tests;
