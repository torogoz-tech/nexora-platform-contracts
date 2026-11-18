/// Closed, credential-free failure codes emitted by the synchronization worker.
#[derive(Clone, Copy)]
pub enum RepositoryJobFailure {
    SourceAccessDenied,
    SourceUnavailable,
    SourceInvalid,
    ProfileUnavailable,
    AnalysisFailed,
    StorageUnavailable,
}
impl RepositoryJobFailure {
    pub fn code(self) -> &'static str {
        match self {
            Self::SourceAccessDenied => "source_access_denied",
            Self::SourceUnavailable => "source_unavailable",
            Self::SourceInvalid => "source_invalid",
            Self::ProfileUnavailable => "profile_unavailable",
            Self::AnalysisFailed => "analysis_failed",
            Self::StorageUnavailable => "storage_unavailable",
        }
    }
}
