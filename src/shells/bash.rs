use super::Shell;

pub struct Bash;

impl Shell for Bash {
    fn app_alias(&self, alias_name: &str) -> String {
        format!(
            r#"
function {alias_name} () {{
    TF_PYTHONIOENCODING=$PYTHONIOENCODING;
    export TF_SHELL=bash;
    export TF_ALIAS={alias_name};
    export TF_SHELL_ALIASES=$(alias);
    export TF_HISTORY=$(fc -ln -10);
    export PYTHONIOENCODING=utf-8;
    TF_CMD=$(
        ffs "$@"
    );
    BLOCK_RES=$?;
    if [ $BLOCK_RES -eq 0 ]; then
        eval "$TF_CMD";
    fi
    unset TF_HISTORY;
    export PYTHONIOENCODING=$TF_PYTHONIOENCODING;
    history -s $TF_CMD;
}}
"#,
            alias_name = alias_name
        )
    }

    fn get_history_file_name(&self) -> String {
        std::env::var("HISTFILE").unwrap_or_else(|_| {
            dirs::home_dir()
                .map(|h| h.join(".bash_history").to_string_lossy().to_string())
                .unwrap_or_else(|| ".bash_history".to_string())
        })
    }
}
