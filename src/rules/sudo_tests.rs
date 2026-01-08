#[cfg(test)]
mod tests {
    use crate::rules::sudo::Sudo;
    use crate::types::Command;
    use crate::rules::Rule;

    #[test]
    fn test_sudo() {
        let rule = Sudo;
        let command = Command::new(
            "apt-get install vim".to_string(),
            "".to_string(),
            "E: Could not open lock file /var/lib/dpkg/lock - open (13: Permission denied)".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "sudo apt-get install vim");
    }
}
