use crate::types::Command;
use crate::rules::brew::BrewUnknownCommand;
use crate::rules::Rule;

#[test]
fn test_match() {
    let rule = BrewUnknownCommand;

    let cmd = Command::new(
        "brew docto".to_string(),
        "".to_string(),
        "Error: Unknown command: docto\nDid you mean doctor?".to_string(),
    );
    assert!(rule.matches(&cmd));

    let cmd = Command::new(
        "brew inst".to_string(),
        "".to_string(),
        "Error: No such command: inst\nDid you mean install?".to_string(),
    );
    assert!(rule.matches(&cmd));

    let cmd = Command::new(
        "npm install".to_string(),
        "".to_string(),
        "Error: Unknown command".to_string(),
    );
    assert!(!rule.matches(&cmd));

    let cmd = Command::new(
        "brew doctor".to_string(),
        "Your system is ready to brew.".to_string(),
        "".to_string(),
    );
    assert!(!rule.matches(&cmd));
}

#[test]
fn test_generate_corrections() {
    let rule = BrewUnknownCommand;

    let cmd = Command::new(
        "brew docto".to_string(),
        "".to_string(),
        "Error: Unknown command: docto\nDid you mean doctor?".to_string(),
    );
    let corrections = rule.generate_corrections(&cmd);
    assert_eq!(corrections.len(), 1);
    assert_eq!(corrections[0].command, "brew doctor");

    let cmd = Command::new(
        "brew inst wget".to_string(),
        "".to_string(),
        "Error: Unknown command: inst\nDid you mean install?".to_string(),
    );
    let corrections = rule.generate_corrections(&cmd);
    assert_eq!(corrections.len(), 1);
    assert_eq!(corrections[0].command, "brew install wget");

    let cmd = Command::new(
        "brew foo".to_string(),
        "".to_string(),
        "Error: Unknown command: foo".to_string(),
    );
    let corrections = rule.generate_corrections(&cmd);
    assert_eq!(corrections.len(), 0);
}
