# FFS (For F*cks Sake) - Development Roadmap

This document outlines the plan to reach parity with `thefuck` and extend functionality using Rust's strengths.

## Phase 1: Core Rules Expansion (The "Bread and Butter")

Goal: Implement the most commonly used rules to make the tool immediately useful.

- [ ] **Python Ecosystem**
    - [ ] `python_execute`: Append `.py` to script files if missing.
    - [ ] `pip_unknown_command`: Fix `pip install` -> `pip install`.
    - [ ] `python_command`: Prepend `python` to scripts without shebang/execution bit.
- [ ] **Package Managers**
    - [ ] `apt_get`: Fix invalid operations or missing sudo.
    - [ ] `brew_unknown_command`: Fix `brew docto` -> `brew doctor`.
    - [ ] `cargo`: Expand current cargo rules (already started).
- [ ] **Shell Utilities**
    - [ ] `cp_create_destination`: Create dest dir if missing.
    - [ ] `rm_dir`: Add `-r` when removing directories.
    - [ ] `ls_all`: `ls` empty -> `ls -a`.
    - [ ] `grep_recursive`: `grep dir` -> `grep -r dir`.
- [ ] **Git**
    - [ ] Expand existing Git rules (e.g., `git merge` conflicts, `git branch` existence).
    - [ ] `git_add`: Fix "pathspec" errors.

## Phase 2: Shell & Platform Support

Goal: Support all major shells and OS nuances.

- [x] **Bash** (Implemented)
- [x] **Fish** (Implemented)
- [x] **Zsh** (Implemented)
- [ ] **Powershell**: Add `src/shells/powershell.rs` and alias logic.
- [ ] **Windows Support**: Ensure path handling and commands (e.g., `dir` vs `ls`) work naturally where possible.

## Phase 3: Advanced UX & Configuration

Goal: Improve user interaction and configurability.

- [ ] **Interactive Selection**:
    - [x] Basic selection with `dialoguer`.
    - [ ] Add "Apply without confirmation" mode (`--yeah`).
    - [ ] Multi-select for rules that might return multiple valid options (rare but possible).
- [ ] **Configuration**:
    - [ ] Full environment variable overrides (`TF_RULES`, `TF_EXCLUDE`, etc.).
    - [ ] Per-rule settings (e.g., `git_push` default remote).
- [ ] **History Manipulation**:
    - [ ] Implement `alter_history` to fix the user's shell history file after a correction (complex, shell-specific).

## Phase 4: Extensibility & Performance

Goal: Allow users to extend the tool and ensure it runs instantly.

- [x] **Rhai Scripting** (Basic implementation done)
    - [ ] Document the API for users.
    - [ ] Add support for "side effect" functions in Rhai.
    - [ ] Expose more helper functions to Rhai (e.g., `which`, regex helpers).
- [ ] **Performance**:
    - [ ] Parallel rule matching (Rayon) if rule count grows > 50.
    - [ ] Timeout handling for slow rules.

## Phase 5: Distribution & CI

- [ ] **CI/CD**: GitHub Actions for testing on Linux/Mac/Windows.
- [ ] **Packaging**: Prepare for `crates.io` release.
- [ ] **Installation Script**: `install.sh` for easy bootstrapping.
