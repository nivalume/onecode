//! Sample data for building and demoing the UI until real tasks come from the
//! agent runtime.

use crate::{
    Author, Effort, Machine, Message, Model, PermissionLevel, Provider, Suggestion, SuggestionKind,
    Task, TaskContext, TaskGroup, TaskGroupKind, TaskStatus,
};

/// The person's laptop, where the sample session and its terminal run.
pub fn machine() -> Machine {
    Machine {
        name: "jackson's MacBook Pro".into(),
        user: "jackson".into(),
        host_name: "MacBook-Pro".into(),
        shell: "zsh".into(),
    }
}

/// The model new messages go to.
pub const MODEL: &str = "Gemini 4 Neon";

/// The reasoning effort new messages use.
pub const EFFORT: Effort = Effort::High;

/// What the agent may do in the open task without asking.
pub const PERMISSION_LEVEL: PermissionLevel = PermissionLevel::FullAccess;

/// Models the person starred, in the order ⌘1–9 picks them.
pub const FAVORITE_MODELS: [&str; 3] = ["Gemini 4 Neon", "GPT-6.2-Sol", "Claude Opus 5.5"];

/// The model catalog, grouped by provider.
pub fn models() -> Vec<Model> {
    let catalog: [(Provider, &[&str]); 3] = [
        (
            Provider::Google,
            &[
                "Gemini 4 Neon",
                "Gemini 4 Carbon",
                "Gemini 4 Argon",
                "Gemini 4 Hydrogen",
                "Gemini 3.8 Flash",
            ],
        ),
        (
            Provider::OpenAi,
            &[
                "GPT-6.2-Sol",
                "GPT-6.2-Astra",
                "GPT-6.1-Sol",
                "GPT-6.1 Terra",
                "GPT-6-Astra",
                "GPT-6-Luna",
            ],
        ),
        (
            Provider::Anthropic,
            &[
                "Claude Opus 5.5",
                "Claude Sonnet 5.5",
                "Claude Haiku 5.5",
                "Claude Fable 5.2",
            ],
        ),
    ];
    catalog
        .into_iter()
        .flat_map(|(provider, names)| {
            names.iter().map(move |name| Model {
                name: (*name).into(),
                provider,
            })
        })
        .collect()
}

/// The open task's conversation: a greeting and the agent's first look at
/// the project, with three suggested next steps.
pub fn conversation() -> Vec<Message> {
    vec![
        Message {
            author: Author::Person {
                name: "Jackson".into(),
            },
            text: "Hi".into(),
            suggestions: Vec::new(),
        },
        Message {
            author: Author::Agent {
                model: MODEL.into(),
                elapsed: "2s".into(),
            },
            text: "Hi. I took a look at jarvis: it’s a JavaScript project, and Git isn’t set \
                   up yet. Tell me what you want to do, or start with one of these."
                .into(),
            suggestions: vec![
                Suggestion {
                    kind: SuggestionKind::SetUpGit,
                    title: "Set up Git".into(),
                    description: "Take a first snapshot so every later change can be \
                                  compared and undone."
                        .into(),
                },
                Suggestion {
                    kind: SuggestionKind::ExploreProject,
                    title: "Get to know the project".into(),
                    description: "Read the code and dependencies, then write a one-page \
                                  overview."
                        .into(),
                },
                Suggestion {
                    kind: SuggestionKind::SaveScripts,
                    title: "Save one-click scripts".into(),
                    description: "Turn your dev and test commands into project actions \
                                  you can run anytime."
                        .into(),
                },
            ],
        },
    ]
}

/// The open task's context: a fresh JavaScript project with no Git, rules or
/// saved commands yet, and the terminal and browser tools on.
pub fn task_context() -> TaskContext {
    TaskContext {
        git_initialized: false,
        changed_files: 0,
        saved_commands: 0,
        has_rules_file: false,
        memories: 0,
        terminal_enabled: true,
        browser_enabled: true,
        mcp_servers: 0,
        context_window_used: 0.02,
    }
}

/// Id of the task the sample session has open.
pub const OPEN_TASK: &str = "getting-started";

/// Three sample groups: one task waiting for the person, one running, and one
/// idle in the `jarvis` project.
pub fn task_groups() -> Vec<TaskGroup> {
    vec![
        TaskGroup {
            kind: TaskGroupKind::NeedsYou,
            tasks: vec![Task {
                id: "upgrade-dependencies".into(),
                project: "notes-app".into(),
                title: "Upgrade dependencies".into(),
                detail: Some("approve npm install".into()),
                age: None,
                status: TaskStatus::Waiting,
            }],
        },
        TaskGroup {
            kind: TaskGroupKind::Running,
            tasks: vec![Task {
                id: "migrate-to-vite-7".into(),
                project: "notes-app".into(),
                title: "Migrate to Vite 7".into(),
                detail: Some("Cloud · step 3 of 8".into()),
                age: None,
                status: TaskStatus::Running,
            }],
        },
        TaskGroup {
            kind: TaskGroupKind::Project("jarvis".into()),
            tasks: vec![Task {
                id: OPEN_TASK.into(),
                project: "jarvis".into(),
                title: "Getting started".into(),
                detail: None,
                age: Some("1h".into()),
                status: TaskStatus::Idle,
            }],
        },
    ]
}
