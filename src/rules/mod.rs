use crate::types::{Command, Correction};
use std::fmt::Debug;
use anyhow::Result;

pub trait Rule: Send + Sync + Debug {
    fn matches(&self, command: &Command) -> bool;
    fn generate_corrections(&self, command: &Command) -> Vec<Correction>;
    fn name(&self) -> &str;
}

// Submodules for builtin rules
pub mod cargo;
pub mod git;
pub mod generic;
