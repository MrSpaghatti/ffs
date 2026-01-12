use crate::types::{Command, Correction};
use std::fmt::Debug;

pub trait Rule: Send + Sync + Debug {
    fn matches(&self, command: &Command) -> bool;
    fn generate_corrections(&self, command: &Command) -> Vec<Correction>;
    fn name(&self) -> &str;
}

// Submodules for builtin rules
pub mod cargo;
pub mod git;
pub mod generic;
pub mod mkdir;
pub mod sudo;
pub mod cd;
pub mod python;
pub mod grep;

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
mod grep_tests;
