use crate::types::Command;
use crate::rules::Rule;
use super::git_add::GitAdd;

#[test]
fn test_matches() {
    let rule = GitAdd;

    // Test basic match
    let cmd1 = Command::new(
        "git add oops".to_string(),
        "".to_string(),
        "fatal: pathspec 'oops' did not match any files".to_string(),
    );
    assert!(rule.matches(&cmd1));

    // Test no arguments
    let cmd2 = Command::new(
        "git add".to_string(),
        "".to_string(),
        "Nothing specified, nothing added.\nhint: Maybe you wanted to say 'git add .'?".to_string(),
    );
    assert!(rule.matches(&cmd2));

    // Test generic fallback
    let cmd3 = Command::new(
        "git add missing".to_string(),
        "".to_string(),
        "error: did not match any files".to_string(),
    );
    assert!(rule.matches(&cmd3));

    // Test non-match
    let cmd4 = Command::new(
        "git push".to_string(),
        "".to_string(),
        "fatal: pathspec 'oops' did not match any files".to_string(),
    );
    assert!(!rule.matches(&cmd4));

    // Test non-match correct command
    let cmd5 = Command::new(
        "git add .".to_string(),
        "".to_string(),
        "".to_string(),
    );
    assert!(!rule.matches(&cmd5));
}

#[test]
fn test_generate_corrections() {
    let rule = GitAdd;
    let cmd = Command::new(
        "git add oops".to_string(),
        "".to_string(),
        "fatal: pathspec 'oops' did not match any files".to_string(),
    );

    let corrections = rule.generate_corrections(&cmd);
    assert_eq!(corrections.len(), 2);

    assert_eq!(corrections[0].command, "git add -A");
    assert!(corrections[0].side_effect);
    assert_eq!(corrections[0].priority, 100);

    assert_eq!(corrections[1].command, "git add .");
    assert!(corrections[1].side_effect);
    assert_eq!(corrections[1].priority, 90);
}
