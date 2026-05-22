#[cfg(test)]
mod tests {
    use crate::rules::apt_get::AptGet;
    use crate::types::Command;
    use crate::rules::Rule;

    #[test]
    fn test_apt_get_permission_denied() {
        let rule = AptGet;
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

    #[test]
    fn test_apt_get_invalid_operation_typo() {
        let rule = AptGet;
        let command = Command::new(
            "apt-get instatl vim".to_string(),
            "".to_string(),
            "E: Invalid operation instatl".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "apt-get install vim");
    }

    #[test]
    fn test_apt_get_invalid_operation_uninstall() {
        let rule = AptGet;
        let command = Command::new(
            "apt-get uninstall vim".to_string(),
            "".to_string(),
            "E: Invalid operation uninstall".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "apt-get remove vim");
    }

    #[test]
    fn test_apt_invalid_operation_with_sudo() {
        let rule = AptGet;
        let command = Command::new(
            "sudo apt instatl vim".to_string(),
            "".to_string(),
            "E: Invalid operation instatl".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "sudo apt install vim");
    }

    #[test]
    fn test_apt_not_matching() {
        let rule = AptGet;
        let command = Command::new(
            "apt-get install vim".to_string(),
            "".to_string(),
            "".to_string(),
        );

        assert!(!rule.matches(&command));
    }
}
