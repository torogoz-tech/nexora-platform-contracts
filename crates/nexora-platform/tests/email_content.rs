use nexora_platform::{EmailContentTemplate, EmailTemplate};
use std::collections::{BTreeMap, BTreeSet};
#[test]
fn all_defaults_are_valid_and_have_two_languages_per_event() {
    let rows = EmailContentTemplate::defaults();
    assert_eq!(rows.len(), 20);
    let mut keys = BTreeSet::new();
    for item in rows {
        assert!(keys.insert((item.kind.clone(), item.language.clone())));
        item.validate(&item.subject, &item.body_html).unwrap();
        let values = item
            .parameters
            .iter()
            .map(|p| (p.name.clone(), p.sample.clone()))
            .collect();
        let (subject, html) = item.render(&values).unwrap();
        assert!(!subject.contains("{{"));
        assert!(!html.contains("{{"));
    }
}
#[test]
fn unknown_parameters_removed_required_links_and_subject_headers_are_rejected() {
    let item = EmailContentTemplate::definition("invitation", "es").unwrap();
    for body in [
        "<p>No link</p>",
        "<!-- <a href=\"{{invitation_url}}\">Hidden</a> --> {{expires_at}}",
        "{{invitation_url}} {{expires_at}}",
        "<a href=\"{{invitation_url}}\">OK</a>{{expires_at}}{{unknown}}",
    ] {
        assert!(item.validate(&item.subject, body).is_err(), "{body}");
    }
    assert!(
        item.validate("Subject\r\nBcc: other@example.com", &item.body_html)
            .is_err()
    );
    assert!(item.validate(&item.subject, &"x".repeat(100001)).is_err());
    assert!(EmailContentTemplate::definition("unknown", "es").is_err());
    assert!(EmailContentTemplate::definition("test", "fr").is_err());
}
#[test]
fn values_are_escaped_once_urls_are_verified_and_envelope_keeps_compiled_html() {
    let mut item = EmailContentTemplate::definition("invitation", "en").unwrap();
    item.subject = "{{recipient_name}}".into();
    let mut values: BTreeMap<String, String> = item
        .parameters
        .iter()
        .map(|p| (p.name.clone(), p.sample.clone()))
        .collect();
    values.insert("recipient_name".into(), "<Ana> {{invitation_url}}".into());
    values.insert("inviter_name".into(), "\"><script>alert(1)</script>".into());
    let (subject, body) = item.render(&values).unwrap();
    assert_eq!(subject, "<Ana> {{invitation_url}}");
    assert!(body.contains("&lt;Ana&gt; {{invitation_url}}"));
    assert!(!body.contains("<script>"));
    let envelope = EmailTemplate {
        html: EmailTemplate::DEFAULT_HTML.into(),
        revision: 1,
    };
    let html = envelope.render_html("<Org>", &subject, &body);
    assert!(html.contains("<a href=\"https://nexora.example/"));
    assert!(html.contains("&lt;Org&gt;"));
    for bad in [
        "javascript:alert(1)",
        "data:text/html,test",
        "https://user:password@example.com",
    ] {
        values.insert("invitation_url".into(), bad.into());
        assert!(item.render(&values).is_err());
    }
    values.remove("invitation_url");
    assert!(item.render(&values).is_err());
}
