//! The computer an agent session runs on.

/// The computer a task's session and terminal run on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    /// Name the person gave the computer, e.g. "jackson's MacBook Pro".
    pub name: String,
    /// Account the session's shell runs as, e.g. "jackson".
    pub user: String,
    /// Host name as a shell prompt shows it, e.g. "MacBook-Pro".
    pub host_name: String,
    /// Name of the login shell, e.g. "zsh".
    pub shell: String,
}
