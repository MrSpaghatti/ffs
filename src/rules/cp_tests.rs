use crate::types::Command;
use crate::rules::Rule;
use crate::rules::cp::CpCreateDestination;

#[test]
fn test_matches() {
    let rule = CpCreateDestination;

    // Matches
    assert!(rule.matches(&Command::new(
        "cp file.txt /a/b/c/".to_string(),
        "".to_string(),
        "cp: cannot create regular file '/a/b/c/': No such file or directory".to_string()
    )));

    assert!(rule.matches(&Command::new(
        "cp -r src/ /a/b/c".to_string(),
        "".to_string(),
        "cp: cannot create regular file '/a/b/c': No such file or directory".to_string()
    )));

    assert!(rule.matches(&Command::new(
        "cp file.txt /foo/bar/baz.txt".to_string(),
        "".to_string(),
        "cp: cannot create regular file '/foo/bar/baz.txt': No such file or directory".to_string()
    )));

    // Does not match
    assert!(!rule.matches(&Command::new(
        "mv file.txt /a/b/c/".to_string(),
        "".to_string(),
        "mv: cannot move 'file.txt' to '/a/b/c/': No such file or directory".to_string()
    )));

    assert!(!rule.matches(&Command::new(
        "cp file.txt /a/b/c/".to_string(),
        "".to_string(),
        "cp: missing destination file operand after 'file.txt'".to_string()
    )));
}

#[test]
fn test_generate_corrections() {
    let rule = CpCreateDestination;

    // File copy to non-existent directory
    let cmd = Command::new(
        "cp file.txt /foo/bar/baz.txt".to_string(),
        "".to_string(),
        "cp: cannot create regular file '/foo/bar/baz.txt': No such file or directory".to_string()
    );
    let corrections = rule.generate_corrections(&cmd);
    assert_eq!(corrections.len(), 1);
    assert_eq!(corrections[0].command, "mkdir -p /foo/bar && cp file.txt /foo/bar/baz.txt");

    // Copy to a directory with trailing slash
    let cmd2 = Command::new(
        "cp file.txt /foo/bar/baz/".to_string(),
        "".to_string(),
        "cp: cannot create regular file '/foo/bar/baz/': No such file or directory".to_string()
    );
    let corrections2 = rule.generate_corrections(&cmd2);
    assert_eq!(corrections2.len(), 1);
    assert_eq!(corrections2[0].command, "mkdir -p /foo/bar/baz/ && cp file.txt /foo/bar/baz/");

    // Copy with flags
    let cmd3 = Command::new(
        "cp -r src/ /foo/bar/".to_string(),
        "".to_string(),
        "cp: cannot create directory '/foo/bar/': No such file or directory".to_string()
    );
    let corrections3 = rule.generate_corrections(&cmd3);
    assert_eq!(corrections3.len(), 1);
    assert_eq!(corrections3[0].command, "mkdir -p /foo/bar/ && cp -r src/ /foo/bar/");

    // Needs quoting
    let cmd4 = Command::new(
        "cp file.txt '/foo/bar baz/qux.txt'".to_string(),
        "".to_string(),
        "cp: cannot create regular file '/foo/bar baz/qux.txt': No such file or directory".to_string()
    );
    let corrections4 = rule.generate_corrections(&cmd4);
    assert_eq!(corrections4.len(), 1);
    assert_eq!(corrections4[0].command, "mkdir -p '/foo/bar baz' && cp file.txt '/foo/bar baz/qux.txt'");
}
