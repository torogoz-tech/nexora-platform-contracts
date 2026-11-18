use nexora_platform::{OpenAiConfiguration, OpenAiEndpoint, SmtpConfiguration};
#[test]
fn gateways_keep_tls_and_bearer_credentials_out_of_urls() {
    assert_eq!(
        OpenAiEndpoint::normalize("https://gateway.example.com/corporate/v1/").unwrap(),
        "https://gateway.example.com/corporate/v1/responses"
    );
    assert_eq!(
        OpenAiEndpoint::normalize("https://api.openai.com/v1/responses").unwrap(),
        "https://api.openai.com/v1/responses"
    );
    for endpoint in [
        "http://gateway.example.com/v1",
        "https://user:secret@gateway.example.com/v1",
        "https://gateway.example.com/v1?key=x",
        "https://gateway.example.com/v1#x",
    ] {
        assert!(OpenAiEndpoint::normalize(endpoint).is_err());
    }
    let config = OpenAiConfiguration {
        endpoint: "https://gateway.example.com/v1".into(),
        model: "corporate/model-v1".into(),
        embedding_model: "text-embedding-3-small".into(),
        effort: "medium".into(),
        enabled: true,
    };
    assert!(config.validate().is_ok());
}
#[test]
fn smtp_rejects_plaintext_and_header_injection() {
    let mut config = SmtpConfiguration {
        host: "smtp.example.com".into(),
        port: 587,
        username: "user".into(),
        sender_email: "nexora@example.com".into(),
        sender_name: "Nexora".into(),
        tls_mode: "starttls".into(),
        enabled: true,
    };
    assert!(config.validate().is_ok());
    config.tls_mode = "none".into();
    assert!(config.validate().is_err());
    config.tls_mode = "tls".into();
    config.sender_name = "Name\r\nBcc: bad@example.com".into();
    assert!(config.validate().is_err());
    config.sender_name = "Name".into();
    config.sender_email = "not-an-email".into();
    assert!(config.validate().is_err());
}
