//! Models the agent can run on, how hard they think, and what the agent may
//! do without asking.

/// A model the agent can run on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    /// Unique display name, e.g. "Gemini 4 Neon".
    pub name: String,
    pub provider: Provider,
}

/// The company that serves a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Google,
    OpenAi,
    Anthropic,
}

impl Provider {
    pub const ALL: [Provider; 3] = [Provider::Google, Provider::OpenAi, Provider::Anthropic];

    /// The provider's own name for itself.
    pub fn name(self) -> &'static str {
        match self {
            Provider::Google => "Google",
            Provider::OpenAi => "OpenAI",
            Provider::Anthropic => "Anthropic",
        }
    }
}

/// How long the model reasons before answering; slower is more thorough.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effort {
    Low,
    Medium,
    High,
    Extra,
}

impl Effort {
    pub const ALL: [Effort; 4] = [Effort::Low, Effort::Medium, Effort::High, Effort::Extra];
}

/// How much the agent may do in a task before asking the person.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionLevel {
    AskFirst,
    AutoEdit,
    Auto,
    FullAccess,
}

/// Something the agent can do to the person's project or machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Read,
    Edit,
    Run,
    Web,
}

/// Whether the agent does a [`Capability`] on its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Autonomy {
    OnItsOwn,
    /// On its own only when the agent judges the action safe.
    SafeOnly,
    AsksFirst,
}

impl PermissionLevel {
    pub const ALL: [PermissionLevel; 4] = [
        PermissionLevel::AskFirst,
        PermissionLevel::AutoEdit,
        PermissionLevel::Auto,
        PermissionLevel::FullAccess,
    ];

    /// Whether the agent may use `capability` without asking at this level.
    pub fn autonomy(self, capability: Capability) -> Autonomy {
        use Autonomy::{AsksFirst, OnItsOwn, SafeOnly};
        match (self, capability) {
            (_, Capability::Read) => OnItsOwn,
            (PermissionLevel::FullAccess, _) => OnItsOwn,
            (PermissionLevel::AutoEdit | PermissionLevel::Auto, Capability::Edit) => OnItsOwn,
            (PermissionLevel::Auto, Capability::Run) => SafeOnly,
            _ => AsksFirst,
        }
    }
}

impl Capability {
    pub const ALL: [Capability; 4] = [
        Capability::Read,
        Capability::Edit,
        Capability::Run,
        Capability::Web,
    ];
}

#[cfg(test)]
mod tests {
    use super::Autonomy::{AsksFirst, OnItsOwn, SafeOnly};
    use super::*;

    #[test]
    fn each_level_grants_the_designed_capabilities() {
        let rows: Vec<Vec<Autonomy>> = PermissionLevel::ALL
            .iter()
            .map(|level| Capability::ALL.map(|c| level.autonomy(c)).to_vec())
            .collect();
        assert_eq!(
            rows,
            [
                vec![OnItsOwn, AsksFirst, AsksFirst, AsksFirst],
                vec![OnItsOwn, OnItsOwn, AsksFirst, AsksFirst],
                vec![OnItsOwn, OnItsOwn, SafeOnly, AsksFirst],
                vec![OnItsOwn, OnItsOwn, OnItsOwn, OnItsOwn],
            ]
        );
    }
}
