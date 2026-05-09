use crate::types::Command;
use crate::rules::Rule;
use crate::rules::ls::LsAll;

#[test]
fn test_ls_all_matches() {
    let rule = LsAll;

    assert!(rule.matches(&Command::new("ls".to_string(), "".to_string(), "".to_string())));
    assert!(rule.matches(&Command::new("ls -l".to_string(), "total 0".to_string(), "".to_string())));
    assert!(rule.matches(&Command::new("ls --color".to_string(), "".to_string(), "".to_string())));

    assert!(!rule.matches(&Command::new("ls dir".to_string(), "".to_string(), "".to_string())));
    assert!(!rule.matches(&Command::new("ls -a".to_string(), "".to_string(), "".to_string())));
    assert!(!rule.matches(&Command::new("ls -la".to_string(), "total 0".to_string(), "".to_string())));
    assert!(!rule.matches(&Command::new("ls --all".to_string(), "".to_string(), "".to_string())));
    assert!(!rule.matches(&Command::new("ls".to_string(), "file.txt".to_string(), "".to_string())));
    assert!(!rule.matches(&Command::new("ls".to_string(), "".to_string(), "ls: cannot access 'dir': No such file or directory".to_string())));
}

#[test]
fn test_ls_all_corrections() {
    let rule = LsAll;

    let corrections = rule.generate_corrections(&Command::new("ls".to_string(), "".to_string(), "".to_string()));
    assert_eq!(corrections[0].command, "ls -a");

    let corrections = rule.generate_corrections(&Command::new("ls -l".to_string(), "total 0".to_string(), "".to_string()));
    assert_eq!(corrections[0].command, "ls -la");

    let corrections = rule.generate_corrections(&Command::new("ls --color".to_string(), "".to_string(), "".to_string()));
    assert_eq!(corrections[0].command, "ls -a --color");
}
