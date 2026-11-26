//! OneCode domain model, independent of any UI or agent runtime.

mod conversation;
mod machine;
mod model;
pub mod sample;
mod task;

pub use conversation::{Author, Message, Suggestion, SuggestionKind};
pub use machine::Machine;
pub use model::{Autonomy, Capability, Effort, Model, PermissionLevel, Provider};
pub use task::{Task, TaskContext, TaskGroup, TaskGroupKind, TaskStatus};
