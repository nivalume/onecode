//! Tasks the person hands to the agent, and how they are grouped for review.

/// A unit of work handed to the agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// Stable identity, unique across all of the person's tasks.
    pub id: String,
    /// Name of the project the task works in.
    pub project: String,
    pub title: String,
    /// One line on where the task runs and how far it got, e.g. "Cloud · step 3 of 8".
    // shortcut: preformatted text, replace with structured fields once tasks
    // come from the agent runtime.
    pub detail: Option<String>,
    /// Time since the last activity, formatted for display, e.g. "1h".
    // shortcut: preformatted text, replace with a timestamp once tasks come
    // from the agent runtime.
    pub age: Option<String>,
    pub status: TaskStatus,
}

/// Whether the agent is working on a task, waiting for the person, or done.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// The agent is blocked until the person answers, e.g. approves a command.
    Waiting,
    /// The agent is working on it now.
    Running,
    /// Nothing is in progress.
    Idle,
}

/// Tasks shown together in the task list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskGroup {
    pub kind: TaskGroupKind,
    pub tasks: Vec<Task>,
}

/// What a [`TaskGroup`] collects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskGroupKind {
    /// Tasks waiting for the person, across projects.
    NeedsYou,
    /// Tasks the agent is working on, across projects.
    Running,
    /// The remaining tasks of one project, by project name.
    Project(String),
}

/// What a task can see and use: the state of its project, the tools it may
/// run, and how full the model's context window is.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskContext {
    /// Whether the project is under version control.
    pub git_initialized: bool,
    /// Files changed since the last snapshot.
    pub changed_files: usize,
    /// Dev and test commands saved as one-click project actions.
    pub saved_commands: usize,
    /// Whether the project has an AGENTS.md with rules for the agent.
    pub has_rules_file: bool,
    /// Corrections the person made that the agent keeps.
    pub memories: usize,
    pub terminal_enabled: bool,
    pub browser_enabled: bool,
    /// Connected MCP servers.
    pub mcp_servers: usize,
    /// Share of the model's context window in use, from 0.0 to 1.0.
    pub context_window_used: f32,
}
