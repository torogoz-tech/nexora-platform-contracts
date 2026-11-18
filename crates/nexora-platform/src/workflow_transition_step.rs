use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkflowTransitionStep {
    RequireApproval,
    VerifyPush {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository_id: Option<String>,
    },
    CreatePullRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository_id: Option<String>,
        #[serde(default)]
        source_branch: String,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        source_from_task: bool,
        #[serde(default)]
        target_branch: String,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        target_from_parent: bool,
    },
    MergePullRequest {
        #[serde(default)]
        repository_id: Option<String>,
        #[serde(default)]
        target_branch: String,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        target_from_parent: bool,
    },
    GenerateContributorRatings {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository_id: Option<String>,
    },
    CreateCodeReview {
        #[serde(default)]
        repository_id: Option<String>,
    },
    CreateBranch {
        #[serde(default)]
        repository_id: Option<String>,
        base_branch: String,
    },
    VerifyPipeline {
        #[serde(default)]
        repository_id: Option<String>,
        #[serde(default)]
        workflow: Option<String>,
    },
    VerifyPullRequestMerged {
        #[serde(default)]
        repository_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target_branch: Option<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        target_from_parent: bool,
    },
    VerifyDeployment {
        #[serde(default)]
        repository_id: Option<String>,
        environment: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target_branch: Option<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        target_from_parent: bool,
    },
    PublishRelease,
    Handoff {
        workflow_id: String,
    },
    AdvanceState,
    Notify {
        channel: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        audience: Option<String>,
    },
}
