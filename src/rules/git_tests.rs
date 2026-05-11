#[cfg(test)]
mod tests {
    use crate::rules::git::{GitPush, GitNoCommand, GitCheckout, GitMerge, GitBranchExists, GitStash};
    use crate::types::Command;
    use crate::rules::Rule;

    #[test]
    fn test_git_push() {
        let rule = GitPush;
        let command = Command::new(
            "git push".to_string(),
            "".to_string(),
            "fatal: The current branch master has no upstream branch.\nTo push the current branch and set the remote as upstream, use\n\n    git push --set-upstream origin master\n".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "git push --set-upstream origin master");
    }

    #[test]
    fn test_git_no_command_typo() {
        let rule = GitNoCommand;
        let command = Command::new(
            "git brnch".to_string(),
            "".to_string(),
            "git: 'brnch' is not a git command. See 'git --help'.\n\nDid you mean this?\n\tbranch".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "git branch");
    }

    #[test]
    fn test_git_checkout_new_branch() {
        let rule = GitCheckout;
        let command = Command::new(
            "git checkout feature".to_string(),
            "".to_string(),
            "error: pathspec 'feature' did not match any file(s) known to git".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "git checkout -b feature");
    }

    #[test]
    fn test_git_merge() {
        let rule = GitMerge;
        let command = Command::new(
            "git merge feature".to_string(),
            "".to_string(),
            "Automatic merge failed; fix conflicts and then commit the result.".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 2);
        assert_eq!(corrections[0].command, "git merge --abort");
        assert_eq!(corrections[1].command, "git mergetool");
    }

    #[test]
    fn test_git_branch_exists() {
        let rule = GitBranchExists;
        let command = Command::new(
            "git branch -d feature".to_string(),
            "".to_string(),
            "error: The branch 'feature' is not fully merged.\nIf you are sure you want to delete it, run 'git branch -D feature'.".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "git branch -D feature");
    }

    #[test]
    fn test_git_stash() {
        let rule = GitStash;
        let command = Command::new(
            "git stash pop".to_string(),
            "".to_string(),
            "Auto-merging file.txt\nCONFLICT (content): Merge conflict in file.txt".to_string(),
        );

        assert!(rule.matches(&command));
        let corrections = rule.generate_corrections(&command);
        assert_eq!(corrections.len(), 1);
        assert_eq!(corrections[0].command, "git stash drop");
    }
}
