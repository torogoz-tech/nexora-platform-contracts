use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    RepositorySync,
    RepositoryIndex,
    TaskTransitionResume,
    ReleaseComposition,
    KnowledgeSynthesis,
    KnowledgeProcessing,
    DailyDeveloperAnalysis,
    EmailNotification,
}

impl JobKind {
    pub fn queue_name(self) -> &'static str {
        self.routing_key()
    }

    pub fn routing_key(self) -> &'static str {
        match self {
            Self::RepositorySync | Self::RepositoryIndex => "nexora.jobs.analysis",
            Self::TaskTransitionResume | Self::ReleaseComposition => "nexora.jobs.release",
            Self::KnowledgeSynthesis | Self::KnowledgeProcessing | Self::DailyDeveloperAnalysis => {
                "nexora.jobs.knowledge"
            }
            Self::EmailNotification => "nexora.jobs.notification",
        }
    }
}
