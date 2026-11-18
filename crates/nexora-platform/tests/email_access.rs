use nexora_platform::EmailAccessSettings;
#[test]
fn links_only_use_a_configured_https_origin_or_local_development_origin() {
    assert_eq!(
        EmailAccessSettings::validate("https://nexora.com/").unwrap(),
        "https://nexora.com"
    );
    assert_eq!(
        EmailAccessSettings::validate("http://localhost:5173").unwrap(),
        "http://localhost:5173"
    );
    assert_eq!(EmailAccessSettings::validate("").unwrap(), "");
    for value in [
        "http://example.com",
        "https://user:password@example.com",
        "https://example.com/path",
        "https://example.com/?next=bad",
        "https://example.com/#token=x",
        "javascript:alert(1)",
        "http://localhost.evil.example",
    ] {
        assert!(EmailAccessSettings::validate(value).is_err());
    }
}
