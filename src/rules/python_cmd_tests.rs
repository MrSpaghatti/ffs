use crate::types::Command;
use crate::rules::Rule;
use crate::rules::python_cmd::PythonCommand;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_python_cmd_matches() {
    let rule = PythonCommand;
    let dir = tempdir().unwrap();
    
    // Create a script file
    let script_path = dir.path().join("script.py");
    let mut file = File::create(&script_path).unwrap();
    writeln!(file, "print('hello')").unwrap();
    
    let path_str = script_path.to_str().unwrap();
    
    // Command fails with permission denied
    let cmd = Command::new(
        format!("{} arg1 arg2", path_str),
        "".to_string(),
        "permission denied".to_string(),
    );
    assert!(rule.matches(&cmd));
    
    // Command fails with exec format error
    let cmd = Command::new(
        format!("{} arg1 arg2", path_str),
        "".to_string(),
        "exec format error".to_string(),
    );
    assert!(rule.matches(&cmd));
}

#[test]
fn test_python_cmd_does_not_match_executable() {
    let rule = PythonCommand;
    let dir = tempdir().unwrap();
    
    let script_path = dir.path().join("script.py");
    let mut file = File::create(&script_path).unwrap();
    writeln!(file, "print('hello')").unwrap();
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms).unwrap();
    }
    
    let path_str = script_path.to_str().unwrap();
    let cmd = Command::new(
        path_str.to_string(),
        "".to_string(),
        "permission denied".to_string(),
    );
    assert!(!rule.matches(&cmd));
}

#[test]
fn test_python_cmd_does_not_match_shebang() {
    let rule = PythonCommand;
    let dir = tempdir().unwrap();
    
    let script_path = dir.path().join("script.py");
    let mut file = File::create(&script_path).unwrap();
    writeln!(file, "#!/usr/bin/env python\nprint('hello')").unwrap();
    
    let path_str = script_path.to_str().unwrap();
    let cmd = Command::new(
        path_str.to_string(),
        "".to_string(),
        "permission denied".to_string(),
    );
    assert!(!rule.matches(&cmd));
}

#[test]
fn test_python_cmd_generate_corrections() {
    let rule = PythonCommand;
    let cmd = Command::new("foo.py arg1".to_string(), "".to_string(), "".to_string());
    let corrections = rule.generate_corrections(&cmd);
    assert_eq!(corrections.len(), 1);
    assert_eq!(corrections[0].command, "python foo.py arg1");
    assert_eq!(corrections[0].priority, 100);
}
