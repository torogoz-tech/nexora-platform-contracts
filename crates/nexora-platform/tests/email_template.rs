use nexora_platform::EmailTemplate;
#[test]
fn default_is_valid_and_renderer_escapes_values_without_recursive_substitution() {
    EmailTemplate::validate(EmailTemplate::DEFAULT_HTML).unwrap();
    let template = EmailTemplate {
        html: "<h1>{{subject}}</h1>".to_owned() + EmailTemplate::DEFAULT_HTML,
        revision: 1,
    };
    let html = template.render("<Acme & Co>", "{{body}}", "<script>alert(1)</script>\nNext");
    assert!(html.contains("&lt;Acme &amp; Co&gt;"));
    assert!(html.contains("{{body}}"));
    assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;<br>Next"));
    assert!(!html.contains("<script>"));
}
#[test]
fn template_requires_one_body_and_known_tokens_and_a_bounded_size() {
    for invalid in [
        "<p>Hello</p>",
        "{{body}}{{body}}",
        "{{unknown}}{{body}}",
        "{{body}}{{broken",
    ] {
        assert!(EmailTemplate::validate(invalid).is_err(), "{invalid}");
    }
    assert!(EmailTemplate::validate(&format!("{{{{body}}}}{}", "a".repeat(100000))).is_err());
    assert!(EmailTemplate::validate("<div>{{body}}</div>").is_ok());
}

#[test]
fn subject_is_not_a_heading_unless_the_editable_html_explicitly_includes_it() {
    let template = EmailTemplate {
        html: EmailTemplate::DEFAULT_HTML.into(),
        revision: 1,
    };
    let body = "<p>Hello</p>";
    let html = template.render_html("Acme", "A subject that must stay out of the body", body);
    assert!(!html.contains("A subject that must stay out of the body"));
    assert!(!html.contains("<h1"));
    assert!(html.contains(body));
    let custom = EmailTemplate {
        html: "<header>{{subject}}</header><main>{{body}}</main>".into(),
        revision: 1,
    };
    assert_eq!(
        custom.render_html("Acme", "<Chosen title>", body),
        "<header>&lt;Chosen title&gt;</header><main><p>Hello</p></main>"
    );
}
