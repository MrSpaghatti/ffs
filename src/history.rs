use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn alter_history(old_cmd: &str, new_cmd: &str, shell: &str) -> Result<()> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    
    let history_file: Option<PathBuf> = match shell {
        "bash" => Some(home.join(".bash_history")),
        "zsh" => Some(home.join(".zsh_history")),
        "fish" => Some(home.join(".local/share/fish/fish_history")),
        _ => None,
    };

    if let Some(path) = history_file {
        alter_history_file(old_cmd, new_cmd, shell, &path)?;
    }

    Ok(())
}

fn alter_history_file(old_cmd: &str, new_cmd: &str, shell: &str, path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    for i in (0..lines.len()).rev() {
        if shell == "fish" {
            let cmd_prefix = "- cmd: ";
            if lines[i].starts_with(cmd_prefix) {
                let cmd_content = &lines[i][cmd_prefix.len()..];
                if cmd_content == old_cmd {
                    let new_line = format!("- cmd: {}", new_cmd);
                    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
                    new_lines[i] = new_line;
                    fs::write(path, new_lines.join("\n") + "\n")?;
                    return Ok(());
                }
            }
        } else if shell == "zsh" {
            if let Some(idx) = lines[i].find(';') {
                let cmd_content = &lines[i][idx + 1..];
                if cmd_content == old_cmd {
                    let new_line = format!("{};{}", &lines[i][..idx], new_cmd);
                    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
                    new_lines[i] = new_line;
                    fs::write(path, new_lines.join("\n") + "\n")?;
                    return Ok(());
                }
            } else if lines[i] == old_cmd {
                let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
                new_lines[i] = new_cmd.to_string();
                fs::write(path, new_lines.join("\n") + "\n")?;
                return Ok(());
            }
        } else {
            if lines[i] == old_cmd {
                let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
                new_lines[i] = new_cmd.to_string();
                fs::write(path, new_lines.join("\n") + "\n")?;
                return Ok(());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_alter_bash_history() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "ls\ngit status\nwrong_cmd\necho 'hello'")?;
        
        let path = file.path().to_path_buf();
        alter_history_file("wrong_cmd", "right_cmd", "bash", &path)?;
        
        let content = fs::read_to_string(&path)?;
        assert_eq!(content, "ls\ngit status\nright_cmd\necho 'hello'\n");
        Ok(())
    }

    #[test]
    fn test_alter_zsh_history() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, ": 1618390886:0;ls\n: 1618390887:0;wrong_cmd\n: 1618390888:0;echo 'hello'")?;
        
        let path = file.path().to_path_buf();
        alter_history_file("wrong_cmd", "right_cmd", "zsh", &path)?;
        
        let content = fs::read_to_string(&path)?;
        assert_eq!(content, ": 1618390886:0;ls\n: 1618390887:0;right_cmd\n: 1618390888:0;echo 'hello'\n");
        Ok(())
    }

    #[test]
    fn test_alter_fish_history() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "- cmd: ls\n  when: 1618390886\n- cmd: wrong_cmd\n  when: 1618390887\n- cmd: echo 'hello'")?;
        
        let path = file.path().to_path_buf();
        alter_history_file("wrong_cmd", "right_cmd", "fish", &path)?;
        
        let content = fs::read_to_string(&path)?;
        assert_eq!(content, "- cmd: ls\n  when: 1618390886\n- cmd: right_cmd\n  when: 1618390887\n- cmd: echo 'hello'\n");
        Ok(())
    }
}
