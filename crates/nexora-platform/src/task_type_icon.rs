use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskTypeIcon {
    #[default]
    ListTodo,
    Bug,
    Lightbulb,
    Search,
    Wrench,
    ShieldAlert,
    GitPullRequest,
    FileCheck,
    MessageSquare,
    FlaskConical,
    Code,
    Flag,
}
impl TaskTypeIcon {
    pub fn is_default(&self) -> bool {
        matches!(self, Self::ListTodo)
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ListTodo => "list-todo",
            Self::Bug => "bug",
            Self::Lightbulb => "lightbulb",
            Self::Search => "search",
            Self::Wrench => "wrench",
            Self::ShieldAlert => "shield-alert",
            Self::GitPullRequest => "git-pull-request",
            Self::FileCheck => "file-check",
            Self::MessageSquare => "message-square",
            Self::FlaskConical => "flask-conical",
            Self::Code => "code",
            Self::Flag => "flag",
        }
    }
}
