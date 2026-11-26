//! The conversation between the person and the agent inside a task.

/// One turn in a task's conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub author: Author,
    pub text: String,
    /// Next steps the agent offers with this message, in the order the
    /// person picks them by number.
    pub suggestions: Vec<Suggestion>,
}

/// Who wrote a message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Author {
    /// The person, by display name.
    Person { name: String },
    /// The agent, with the model that answered and how long it took.
    Agent {
        model: String,
        /// Formatted for display, e.g. "2s".
        // shortcut: preformatted text, replace with a duration once messages
        // come from the agent runtime.
        elapsed: String,
    },
}

/// A next step the agent offers; the person picks one by its number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub kind: SuggestionKind,
    pub title: String,
    pub description: String,
}

/// What a suggestion would do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuggestionKind {
    /// Initialize version control with a first snapshot.
    SetUpGit,
    /// Read the project and write an overview of it.
    ExploreProject,
    /// Save the project's dev and test commands as one-click actions.
    SaveScripts,
}
