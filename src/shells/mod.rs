
pub trait Shell {
    fn app_alias(&self, alias_name: &str) -> String;
    fn get_history_file_name(&self) -> String;
}

pub mod bash;
pub mod fish;

pub use self::bash::Bash;
pub use self::fish::Fish;
