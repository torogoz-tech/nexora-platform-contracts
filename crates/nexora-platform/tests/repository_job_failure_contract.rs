use nexora_platform::RepositoryJobFailure;

#[test]
fn repository_job_failures_preserve_closed_stable_error_codes() {
    let failures = [
        (
            RepositoryJobFailure::SourceAccessDenied,
            "source_access_denied",
        ),
        (
            RepositoryJobFailure::SourceUnavailable,
            "source_unavailable",
        ),
        (RepositoryJobFailure::SourceInvalid, "source_invalid"),
        (
            RepositoryJobFailure::ProfileUnavailable,
            "profile_unavailable",
        ),
        (RepositoryJobFailure::AnalysisFailed, "analysis_failed"),
        (
            RepositoryJobFailure::StorageUnavailable,
            "storage_unavailable",
        ),
    ];

    for (failure, expected_code) in failures {
        assert_eq!(failure.code(), expected_code);
    }
}
