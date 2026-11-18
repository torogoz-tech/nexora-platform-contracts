//! Workflow-led Nexora platform domain, independent of HTTP, persistence and AI.
mod feature_complete;
pub use feature_complete::*;
mod task_list_query;
mod task_priority;
pub use task_list_query::*;
pub use task_priority::*;
mod task_purpose;
pub use task_purpose::*;
mod task_type_create;
pub use task_type_create::*;
mod task_type;
pub use task_type::*;
mod backlog_create;
pub use backlog_create::*;
mod backlog_change;
pub use backlog_change::*;
mod backlog;
pub use backlog::*;
mod backlog_reorder;
pub use backlog_reorder::*;
mod task_create;
pub use task_create::*;
mod task_block_change;
pub use task_block_change::*;
mod task_change;
pub use task_change::*;
mod task_transition_request;
pub use task_transition_request::*;
mod work_task;
pub use work_task::*;
mod task_summary;
pub use task_summary::*;
mod task_page;
pub use task_page::*;
mod task_history_entry;
pub use task_history_entry::*;
mod task_history_page;
pub use task_history_page::*;
mod workflow_evaluation_context;
mod workflow_guard_evaluator;
pub use workflow_evaluation_context::*;
mod workflow_version_page;
mod workflow_version_summary;
pub use workflow_version_page::*;
pub use workflow_version_summary::*;
mod workflow_change;
mod workflow_create;
mod workflow_definition;
mod workflow_page;
mod workflow_publish;
mod workflow_resource_kind;
mod workflow_summary;
mod workflow_version;
pub use workflow_change::*;
pub use workflow_create::*;
pub use workflow_definition::*;
pub use workflow_page::*;
pub use workflow_publish::*;
pub use workflow_resource_kind::*;
pub use workflow_summary::*;
pub use workflow_version::*;
mod workflow_graph;
mod workflow_guard;
mod workflow_state;
mod workflow_transition;
pub use workflow_graph::*;
pub use workflow_guard::*;
pub use workflow_state::*;
pub use workflow_transition::*;
mod repository_job;
pub use repository_job::*;
mod repository_job_change;
pub use repository_job_change::*;
mod repository_job_kind;
pub use repository_job_kind::*;
mod job_kind;
#[cfg(test)]
mod job_wakeup_tests;
pub use job_kind::*;
mod job_wakeup;
pub use job_wakeup::*;
mod job_wakeup_wire;
mod repository_job_request;
pub use repository_job_request::*;
mod repository_job_state;
pub use repository_job_state::*;
mod platform_error;
pub use platform_error::*;
mod actor_kind;
pub use actor_kind::*;
mod auth_method;
pub use auth_method::*;
mod capability;
pub use capability::*;
mod capability_grant;
pub use capability_grant::*;
mod key_scope;
pub use key_scope::*;
mod auth_context;
pub use auth_context::*;
mod domain_text;
pub use domain_text::*;
mod actor;
pub use actor::*;
mod project;
pub use project::*;
mod api_key_metadata;
pub use api_key_metadata::*;
mod issued_api_key;
pub use issued_api_key::*;
mod issued_session;
pub use issued_session::*;
mod bootstrap_result;
pub use bootstrap_result::*;

mod project_change;
pub use project_change::*;
mod project_page;
pub use project_page::*;

mod git_provider;
pub use git_provider::*;

mod repository_authorization;
pub use repository_authorization::*;

mod repository_registration;
pub use repository_registration::*;

mod project_repository;
pub use project_repository::*;
mod repository_source_kind;
pub use repository_source_kind::*;
mod local_repository_source_info;
pub use local_repository_source_info::*;
mod local_source_state;
pub use local_source_state::*;
mod local_snapshot_origin;
pub use local_snapshot_origin::*;
mod local_snapshot_adoption;
pub use local_snapshot_adoption::*;
mod local_snapshot_publication;
pub use local_snapshot_publication::*;
mod local_source_change;
pub use local_source_change::*;

mod repository_page;
pub use repository_page::*;

mod repository_change;
pub use repository_change::*;

mod oidc_provider_registration;
pub use oidc_provider_registration::*;

mod oidc_provider;
pub use oidc_provider::*;

mod oidc_user_registration;
pub use oidc_user_registration::*;

mod oidc_identity_assignment;
pub use oidc_identity_assignment::*;

mod authentication_policy;
pub use authentication_policy::*;

mod authentication_policy_change;
pub use authentication_policy_change::*;

mod oidc_authorization;
pub use oidc_authorization::*;

mod gitlab_client_registration;
pub use gitlab_client_registration::*;

mod gitlab_configuration;
pub use gitlab_configuration::*;

mod git_connection_status;
pub use git_connection_status::*;

mod git_connection;
pub use git_connection::*;
mod git_connection_page;
pub use git_connection_page::*;
mod remote_repository_metadata;
pub use remote_repository_metadata::*;
mod remote_repository_page;
pub use remote_repository_page::*;
mod git_configuration_state_change;
pub use git_configuration_state_change::*;

mod git_repository_authorization;
pub use git_repository_authorization::*;

mod git_authorization;
pub use git_authorization::*;

mod github_app_registration;
pub use github_app_registration::*;

mod github_configuration;
pub use github_configuration::*;

mod github_consent;
pub use github_consent::*;

mod github_installation;
pub use github_installation::*;

mod github_installation_page;
pub use github_installation_page::*;

mod github_installation_selection;
pub use github_installation_selection::*;

mod github_manifest_request;
pub use github_manifest_request::*;
mod github_manifest_authorization;
pub use github_manifest_authorization::*;

mod gitlab_webhook_registration;
pub use gitlab_webhook_registration::*;

mod gitlab_webhook;
pub use gitlab_webhook::*;

mod webhook_receipt;
pub use webhook_receipt::*;
mod repository_binding;
pub use repository_binding::*;
mod repository_job_failure;
pub use repository_job_failure::*;

mod task_approval_grant;
pub use task_approval_grant::*;
mod task_approval;
pub use task_approval::*;

mod task_approval_revoke;
pub use task_approval_revoke::*;

mod task_dependency_create;
pub use task_dependency_create::*;
mod task_dependency_change;
pub use task_dependency_change::*;
mod task_dependency;
pub use task_dependency::*;
mod task_dependency_page;
pub use task_dependency_page::*;
mod task_comment_create;
pub use task_comment_create::*;
mod task_comment;
pub use task_comment::*;
mod task_comment_page;
pub use task_comment_page::*;
mod task_dependency_kind;
pub use task_dependency_kind::*;
mod task_dependency_status;
pub use task_dependency_status::*;
mod task_dependency_query;
pub use task_dependency_query::*;

mod task_template_create;
pub use task_template_create::*;
mod task_template_change;
pub use task_template_change::*;
mod task_template_fields;
pub use task_template_fields::*;
mod task_template_publish;
pub use task_template_publish::*;
mod task_template_apply;
pub use task_template_apply::*;
mod task_template;
pub use task_template::*;
mod task_template_version;
pub use task_template_version::*;
mod task_template_version_summary;
pub use task_template_version_summary::*;
mod task_template_version_page;
pub use task_template_version_page::*;
mod task_template_page;
pub use task_template_page::*;

mod document_origin;
pub use document_origin::*;

mod requirement_body;
pub use requirement_body::*;

mod business_rule_body;
pub use business_rule_body::*;

mod decision_body;
pub use decision_body::*;

mod architecture_declaration;
pub use architecture_declaration::*;

mod project_document_body;
pub use project_document_body::*;

mod document_content;
pub use document_content::*;

mod project_document_create;
pub use project_document_create::*;

mod project_document_revise;
pub use project_document_revise::*;

mod project_document_change;
pub use project_document_change::*;

mod document_transition_request;
pub use document_transition_request::*;

mod project_document_publish;
pub use project_document_publish::*;

mod project_document;
pub use project_document::*;

mod project_document_version;
pub use project_document_version::*;

mod document_version_summary;
pub use document_version_summary::*;

mod document_version_page;
pub use document_version_page::*;

mod document_publication;
pub use document_publication::*;

mod document_export;
pub use document_export::*;

mod document_list_query;
pub use document_list_query::*;

mod document_summary;
pub use document_summary::*;

mod document_page;
pub use document_page::*;

mod document_history_event;
pub use document_history_event::*;

mod document_history_page;
pub use document_history_page::*;

mod document_publication_page;
pub use document_publication_page::*;

mod project_context_manifest_create;
pub use project_context_manifest_create::*;

mod project_context_member;
pub use project_context_member::*;

mod project_context_manifest;
pub use project_context_manifest::*;

mod project_context_status;
pub use project_context_status::*;

mod document_evidence_input;
pub use document_evidence_input::*;

mod document_evidence;
pub use document_evidence::*;

mod document_link_kind;
pub use document_link_kind::*;

mod document_link_target;
pub use document_link_target::*;

mod document_link_create;
pub use document_link_create::*;

mod document_link_remove;
pub use document_link_remove::*;

mod document_link;
pub use document_link::*;

mod document_link_page;
pub use document_link_page::*;

mod context_document_reference;
pub use context_document_reference::*;

mod project_context_package_create;
pub use project_context_package_create::*;

mod package_document_snapshot;
pub use package_document_snapshot::*;

mod project_context_package_content;
pub use project_context_package_content::*;

mod project_context_package;
pub use project_context_package::*;

mod context_package_export;
pub use context_package_export::*;

mod approval_request_target;
pub use approval_request_target::*;

mod approval_request_create;
pub use approval_request_create::*;

mod approval_request_status;
pub use approval_request_status::*;

mod approval_decision;
pub use approval_decision::*;

mod approval_request_decide;
pub use approval_request_decide::*;

mod approval_request_cancel;
pub use approval_request_cancel::*;

mod approval_request;
pub use approval_request::*;

mod approval_request_summary;
pub use approval_request_summary::*;

mod approval_request_page;
pub use approval_request_page::*;

mod document_approval;
pub use document_approval::*;

mod document_approval_revoke;
pub use document_approval_revoke::*;

mod task_type_change;
pub use task_type_change::*;
mod task_type_page;
pub use task_type_page::*;
mod backlog_page;
pub use backlog_page::*;

mod task_move;
pub use task_move::*;
mod task_workflow_migration;
pub use task_workflow_migration::*;
mod evidence_upload;
pub use evidence_upload::EvidenceUpload;
mod evidence_record;
pub use evidence_record::EvidenceRecord;
mod evidence_content;
pub use evidence_content::EvidenceContent;
mod development_policy;
pub use development_policy::DevelopmentPolicy;
mod development_policy_change;
pub use development_policy_change::DevelopmentPolicyChange;
mod task_work_prepare;
pub use task_work_prepare::TaskWorkPrepare;
mod task_work_branch;
pub use task_work_branch::TaskWorkBranch;
mod task_work_session;
pub use task_work_session::TaskWorkSession;

mod task_work_branch_report;
pub use task_work_branch_report::*;

mod task_work_report;
pub use task_work_report::*;

mod task_work_provenance;
pub use task_work_provenance::*;

mod task_work_report_receipt;
pub use task_work_report_receipt::*;

mod hex_color;
pub use hex_color::HexColor;
mod color_change;
pub use color_change::ColorChange;

mod pull_request_operation;
pub use pull_request_operation::PullRequestOperation;
mod pull_request_query;
pub use pull_request_query::PullRequestQuery;

mod pull_request_link_create;
pub use pull_request_link_create::PullRequestLinkCreate;
mod pull_request_link_remove;
pub use pull_request_link_remove::PullRequestLinkRemove;

mod developer_workflow_assignment;
pub use developer_workflow_assignment::*;
mod developer_workflow_change;
pub use developer_workflow_change::*;
mod developer_workflow_create;
pub use developer_workflow_create::*;
mod developer_workflow_edit;
pub use developer_workflow_edit::*;

mod developer_directory_entry;
pub use developer_directory_entry::*;

mod developer_directory_page;
pub use developer_directory_page::*;

mod developer_board_task;
pub use developer_board_task::*;

mod developer_board_page;
pub use developer_board_page::*;

mod developer_workflow_removal;
pub use developer_workflow_removal::*;

mod visible_task_query;
pub use visible_task_query::*;
mod visible_task;
pub use visible_task::*;
mod visible_task_page;
pub use visible_task_page::*;
mod visible_task_type;
pub use visible_task_type::*;
mod visible_task_type_page;
pub use visible_task_type_page::*;

mod task_participant;
pub use task_participant::*;

mod catalog_task_type;
pub use catalog_task_type::CatalogTaskType;

mod catalog_task_type_page;
pub use catalog_task_type_page::CatalogTaskTypePage;

mod project_task_type_selection;
pub use project_task_type_selection::ProjectTaskTypeSelection;

mod project_task_type_selection_change;
pub use project_task_type_selection_change::ProjectTaskTypeSelectionChange;

mod task_type_icon;
pub use task_type_icon::TaskTypeIcon;

mod catalog_task_type_create;
pub use catalog_task_type_create::CatalogTaskTypeCreate;

mod catalog_task_type_change;
pub use catalog_task_type_change::CatalogTaskTypeChange;

mod installation_language;
pub use installation_language::InstallationLanguage;

mod installation_preferences;
pub use installation_preferences::InstallationPreferences;

mod installation_preferences_change;
pub use installation_preferences_change::InstallationPreferencesChange;

mod organization_profile;
pub use organization_profile::*;

mod organization_profile_change;
pub use organization_profile_change::*;

mod organization_branding;
pub use organization_branding::*;

mod installation_service;
pub use installation_service::*;

mod installation_service_settings;
pub use installation_service_settings::*;

mod installation_service_change;
pub use installation_service_change::*;

mod openai_endpoint;
pub use openai_endpoint::*;

mod openai_configuration;
pub use openai_configuration::*;

mod smtp_configuration;
pub use smtp_configuration::*;

mod notification_recipient;
pub use notification_recipient::*;

mod notification_recipient_change;
pub use notification_recipient_change::*;

mod email_template;
pub use email_template::EmailTemplate;
mod email_template_change;
pub use email_template_change::EmailTemplateChange;
mod email_template_parameter;
pub use email_template_parameter::EmailTemplateParameter;
mod email_content_template;
pub use email_content_template::EmailContentTemplate;
mod email_content_change;
pub use email_content_change::EmailContentChange;
mod email_access_settings;
pub use email_access_settings::EmailAccessSettings;
mod email_access_change;
pub use email_access_change::EmailAccessChange;
mod member_invitation;
pub use member_invitation::MemberInvitation;
mod member_invitation_create;
pub use member_invitation_create::MemberInvitationCreate;
mod member_invitation_change;
pub use member_invitation_change::MemberInvitationChange;
mod member_invitation_accept;
pub use member_invitation_accept::MemberInvitationAccept;

mod oidc_provider_change;
pub use oidc_provider_change::*;

mod member_role;
pub use member_role::*;
mod member_role_assignment;
pub use member_role_assignment::*;
mod member_role_change;
pub use member_role_change::*;

mod feature_link_proposal;
pub use feature_link_proposal::*;

mod feature_task_proposal;
pub use feature_task_proposal::*;

mod feature_proposal_create;
pub use feature_proposal_create::*;

mod feature_proposal_decision;
pub use feature_proposal_decision::*;

mod workflow_icon;
pub use workflow_icon::*;

mod member_position_change;
pub use member_position_change::*;

mod task_lifecycle;
pub use task_lifecycle::TaskLifecycle;
mod task_take;
pub use task_take::TaskTake;
mod pool_task_create;
pub use pool_task_create::PoolTaskCreate;

mod task_subtask_create;
pub use task_subtask_create::TaskSubtaskCreate;

mod task_subtask_change;
pub use task_subtask_change::TaskSubtaskChange;

mod git_author_alias_create;
pub use git_author_alias_create::*;

mod git_author_alias_revoke;
pub use git_author_alias_revoke::*;

mod git_author_query;
pub use git_author_query::*;

mod repository_analysis_profile_change;
pub use repository_analysis_profile_change::*;

mod navigation_order_change;
pub use navigation_order_change::NavigationOrderChange;

mod personal_mcp_key_create;
pub use personal_mcp_key_create::PersonalMcpKeyCreate;

mod best_practice_body;
mod technical_debt_body;
pub use best_practice_body::BestPracticeBody;
pub use technical_debt_body::TechnicalDebtBody;
mod pull_request_review_report;
pub use pull_request_review_report::PullRequestReviewReport;

mod personal_mcp_key_list_query;
pub use personal_mcp_key_list_query::PersonalMcpKeyListQuery;

mod task_knowledge_link;
pub use task_knowledge_link::TaskKnowledgeLink;

mod task_project_change;
pub use task_project_change::TaskProjectChange;

mod nullable_project;

mod organization_document_write;
pub use organization_document_write::*;

mod knowledge_contribution_input;
pub use knowledge_contribution_input::KnowledgeContributionInput;

mod workflow_transition_step;
pub use workflow_transition_step::WorkflowTransitionStep;
mod task_transition_branch;
pub use task_transition_branch::TaskTransitionBranch;

mod workflow_transition_rules;

mod member_identity_summary;
pub use member_identity_summary::MemberIdentitySummary;

mod task_work_log;
pub use task_work_log::*;
mod task_work_log_input;
pub use task_work_log_input::*;

mod task_stage_completion;
pub use task_stage_completion::*;

mod task_code_review_report;
pub use task_code_review_report::TaskCodeReviewReport;

mod task_subtask_kind;
pub use task_subtask_kind::*;

mod task_code_take;
pub use task_code_take::TaskCodeTake;

mod contributor_rating_report;
pub use contributor_rating_report::ContributorRatingReport;
