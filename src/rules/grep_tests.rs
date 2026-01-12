use crate::types::Command;
use crate::rules::grep::GrepRecursive;
use crate::rules::Rule;

#[test]
fn test_grep_recursive_match() {
    let rule = GrepRecursive;

    // Standard grep
    let cmd = Command::new(
        "grep foo dir".to_string(),
        "".to_string(),
        "grep: dir: Is a directory".to_string(),
    );
    assert!(rule.matches(&cmd));

    // egrep
    let cmd = Command::new(
        "egrep foo dir".to_string(),
        "".to_string(),
        "egrep: dir: Is a directory".to_string(),
    );
    assert!(rule.matches(&cmd));

    // fgrep
    let cmd = Command::new(
        "fgrep foo dir".to_string(),
        "".to_string(),
        "fgrep: dir: Is a directory".to_string(),
    );
    assert!(rule.matches(&cmd));
}

#[test]
fn test_grep_recursive_no_match() {
    let rule = GrepRecursive;

    // Not grep
    let cmd = Command::new(
        "ls dir".to_string(),
        "".to_string(),
        "ls: dir: Is a directory".to_string(), // ls doesn't usually say this but for test sake
    );
    assert!(!rule.matches(&cmd));

    // Grep but no error
    let cmd = Command::new(
        "grep foo file".to_string(),
        "foo".to_string(),
        "".to_string(),
    );
    assert!(!rule.matches(&cmd));

    // Grep but different error
    let cmd = Command::new(
        "grep foo file".to_string(),
        "".to_string(),
        "grep: file: No such file or directory".to_string(),
    );
    assert!(!rule.matches(&cmd));
}

#[test]
fn test_grep_recursive_correction() {
    let rule = GrepRecursive;

    let cmd = Command::new(
        "grep foo dir".to_string(),
        "".to_string(),
        "grep: dir: Is a directory".to_string(),
    );
    let corrections = rule.generate_corrections(&cmd);
    assert_eq!(corrections.len(), 1);
    assert_eq!(corrections[0].command, "grep -r foo dir");
}
