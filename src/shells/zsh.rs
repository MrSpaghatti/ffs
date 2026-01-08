use super::Shell;

pub struct Zsh;

impl Shell for Zsh {
    fn app_alias(&self, alias_name: &str) -> String {
        format!(
            r#"
function {alias_name} () {{
    TF_PYTHONIOENCODING=$PYTHONIOENCODING;
    export TF_SHELL=zsh;
    export TF_ALIAS={alias_name};
    export TF_SHELL_ALIASES=$(alias);
    export TF_HISTORY="$(fc -ln -10)";
    export PYTHONIOENCODING=utf-8;
    TF_CMD=$(
        ffs "$@"
    );
    BLOCK_RES=$?;
    if [ $BLOCK_RES -eq 0 ]; then
        eval "$TF_CMD";
    else
        return $BLOCK_RES;
    fi
    unset TF_HISTORY;
    export PYTHONIOENCODING=$TF_PYTHONIOENCODING;
    print -s $TF_CMD;
}}
"#,
            alias_name = alias_name
        )
    }

    fn get_history_file_name(&self) -> String {
        std::env::var("HISTFILE").unwrap_or_else(|_| {
            dirs::home_dir()
                .map(|h| h.join(".zsh_history").to_string_lossy().to_string())
                .unwrap_or_else(|| ".zsh_history".to_string())
        })
    }
}
