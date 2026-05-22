use clap::Parser;
use ffs::types::Command;
use ffs::config::load_config;
use ffs::engine::Engine;
use ffs::shells::{Shell, Bash, Fish, Zsh};
use ffs::rules::{
    cargo::CargoRule,
    git::{GitCheckout, GitPush, GitNoCommand},
    generic::UnknownCommand,
    mkdir::MkdirP,
    sudo::Sudo,
    cd::CdMkdir,
    python::{PythonExecute, PipUnknownCommand},
    python_cmd::PythonCommand,
    grep::GrepRecursive,
    rm::RmDir,
    apt_get::AptGet,
    brew::BrewUnknownCommand,
    cp::CpCreateDestination,
    ls::LsAll,
    git_add::GitAdd,
};
use ffs::scripting::load_rhai_rules;
use ffs::ui::select_correction;
use ffs::utils::get_last_command;
use std::process::{Command as SysCommand, Stdio};
use anyhow::{Result, anyhow};
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ffs")]
#[command(about = "Magnificent app which corrects your previous console command", long_about = None)]
struct Cli {
    #[arg(short, long)]
    alias: Option<String>,

    /// Skip confirmation and apply the first correction
    #[arg(short = 'y', long)]
    yeah: bool,

    /// Enable multi-select mode
    #[arg(short = 'm', long)]
    select_multiple: bool,

    /// Command arguments (captured when used as alias)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 1. Alias Generation
    if let Some(shell_name) = cli.alias {
        let shell: Box<dyn Shell> = match shell_name.as_str() {
            "bash" => Box::new(Bash),
            "fish" => Box::new(Fish),
            "zsh" => Box::new(Zsh),
            _ => return Err(anyhow!("Unsupported shell: {}", shell_name)),
        };
        println!("{}", shell.app_alias("ffs")); // Or 'fuck' if user wants
        return Ok(());
    }

    // 2. Normal Operation (Fix Command)

    // Step A: Get the Last Command
    let script = match get_last_command() {
        Some(s) => s,
        None => {
            eprintln!("No previous command found in TF_HISTORY.");
            return Ok(());
        }
    };

    // Step B: Re-run the failed command to capture output (stdout/stderr)
    eprintln!("{}", format!("Re-running: {}", script).dimmed());

    let output = SysCommand::new("sh")
        .arg("-c")
        .arg(&script)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let (stdout, stderr) = match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string()
        ),
        Err(e) => {
            eprintln!("Failed to re-run command: {}", e);
            return Ok(());
        }
    };

    let command = Command::new(script, stdout, stderr);

    // Step C: Initialize Engine & Load Rules
    let config = load_config()?;
    let mut engine = Engine::new(config.clone());

    // Register builtin rules
    engine.register_rule(Box::new(CargoRule));
    engine.register_rule(Box::new(GitCheckout));
    engine.register_rule(Box::new(GitPush));
    engine.register_rule(Box::new(GitNoCommand));
    engine.register_rule(Box::new(UnknownCommand));
    engine.register_rule(Box::new(MkdirP));
    engine.register_rule(Box::new(Sudo));
    engine.register_rule(Box::new(CdMkdir));
    engine.register_rule(Box::new(PythonExecute));
    engine.register_rule(Box::new(PythonCommand));
    engine.register_rule(Box::new(PipUnknownCommand));
    engine.register_rule(Box::new(GrepRecursive));
    engine.register_rule(Box::new(RmDir));
    engine.register_rule(Box::new(AptGet));
    engine.register_rule(Box::new(BrewUnknownCommand));
    engine.register_rule(Box::new(CpCreateDestination));
    engine.register_rule(Box::new(LsAll));
    engine.register_rule(Box::new(GitAdd));

    // Load Rhai rules
    // Use XDG config home usually, or ~/.config/ffs/rules
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let rules_dir = config_dir.join("ffs").join("rules");

    if rules_dir.exists() {
        let rhai_rules = load_rhai_rules(&rules_dir);
        for rule in rhai_rules {
            engine.register_rule(Box::new(rule));
        }
    }

    // Step D: Get Corrections
    let corrections = engine.get_corrections(&command);

    if corrections.is_empty() {
        eprintln!("No corrections found.");
        return Ok(());
    }

    // Step E: Select Correction
    if cli.yeah || config.require_confirmation == Some(false) {
        if let Some(correction) = corrections.first() {
            print!("{}", correction.command);
        }
    } else {
        let selected = select_correction(&corrections, cli.select_multiple);
        for (i, correction) in selected.iter().enumerate() {
            if i > 0 {
                println!(); // Ensure separated by newline
            }
            print!("{}", correction.command);
        }
    }

    Ok(())
}
