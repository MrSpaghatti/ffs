use super::Shell;

pub struct Fish;

impl Shell for Fish {
    fn app_alias(&self, alias_name: &str) -> String {
        format!(
            r#"
function {alias_name}
    set -x TF_PYTHONIOENCODING $PYTHONIOENCODING
    set -x TF_SHELL fish
    set -x TF_ALIAS {alias_name}
    set -x TF_SHELL_ALIASES (alias)
    set -x TF_HISTORY (history -n 10)
    set -x PYTHONIOENCODING utf-8

    set -l TF_CMD (ffs $argv)

    if test -n "$TF_CMD"
        eval $TF_CMD
        history --merge ^ /dev/null
    end

    set -e TF_HISTORY
    set -x PYTHONIOENCODING $TF_PYTHONIOENCODING
end
"#,
            alias_name = alias_name
        )
    }

    fn get_history_file_name(&self) -> String {
        std::env::var("XDG_DATA_HOME")
            .map(|h| format!("{}/fish/fish_history", h))
            .unwrap_or_else(|_| {
                 dirs::home_dir()
                .map(|h| h.join(".local/share/fish/fish_history").to_string_lossy().to_string())
                .unwrap_or_else(|| ".local/share/fish/fish_history".to_string())
            })
    }
}
