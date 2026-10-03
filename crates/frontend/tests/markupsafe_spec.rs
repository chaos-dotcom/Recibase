//! `markupsafe.rs`: the escaping Jinja2's MarkupSafe does, and the rewrite
//! that turns MiniJinja's spellings back into it.

use minijinja::{AutoEscape, Environment};
use minijinja::value::Value as TemplateValue;

use recibase_frontend::markupsafe::{escape, markupsafe_compat};

/// The five characters MarkupSafe escapes, and three it does not.
const CASES: &[(&str, &str)] = &[
    ("'", "&#39;"),
    ("\"", "&#34;"),
    ("/", "/"),
    ("&", "&amp;"),
    ("<", "&lt;"),
    (">", "&gt;"),
    (
        "it's <a href=\"/x?a=1&b=2\">Tom & Jerry</a>",
        "it&#39;s &lt;a href=&#34;/x?a=1&amp;b=2&#34;&gt;Tom &amp; Jerry&lt;/a&gt;",
    ),
    ("2 Onion, chopped ✓", "2 Onion, chopped ✓"),
];

/// What MiniJinja escapes `value` to, with the auto-escaping `templates.rs`
/// installs. The name has to look like the templates it renders, because
/// MiniJinja's default callback chooses by extension.
fn minijinja_escape(value: &str) -> String {
    let mut env = Environment::new();
    env.set_auto_escape_callback(|_name| AutoEscape::Html);
    env.add_template("escape.html", "{{ value }}")
        .expect("the escape template compiles");
    let template = env.get_template("escape.html").expect("the escape template");
    template
        .render(TemplateValue::from_serialize(serde_json::json!({"value": value})))
        .expect("the escape template renders")
}

/// `markupsafe::escape` and the MiniJinja-plus-`markupsafe_compat` pair agree
/// with each other, and with what Python's MarkupSafe writes.
#[test]
fn escape_matches_markupsafe() {
    for (input, expected) in CASES {
        assert_eq!(escape(input), *expected, "escape({input:?})");
        assert_eq!(
            markupsafe_compat(&minijinja_escape(input)),
            *expected,
            "the rendered {input:?}",
        );
    }
}

/// The rewrite is a rewrite, not an escape: text without MiniJinja's
/// spellings comes back unchanged, and each of the three is replaced.
#[test]
fn markupsafe_compat_rewrites_the_three_entities() {
    assert_eq!(markupsafe_compat("nothing to rewrite"), "nothing to rewrite");
    assert_eq!(markupsafe_compat("&quot;"), "&#34;");
    assert_eq!(markupsafe_compat("&#x27;"), "&#39;");
    assert_eq!(markupsafe_compat("&#x2f;"), "/");
    assert_eq!(
        markupsafe_compat("a &quot;b&quot; &#x27;c&#x27; &#x2f;d"),
        "a &#34;b&#34; &#39;c&#39; /d",
    );
}

/// A value that itself contains one of the spellings is escaped first, so
/// the rewrite cannot corrupt it: `&` has already become `&amp;`.
#[test]
fn a_value_cannot_forge_a_rewritten_entity() {
    for value in ["&#x27;", "&quot;", "&#x2f;"] {
        let rendered = markupsafe_compat(&minijinja_escape(value));
        assert_eq!(rendered, escape(value), "the rendered {value:?}");
        assert_ne!(rendered, value, "{value:?} must not survive unescaped");
    }
}

/// The other half of that argument: no template contains the spellings
/// literally, so the rewrite has nothing else to touch.
#[test]
fn template_sources_are_free_of_the_rewritten_entities() {
    for (name, source) in recibase_frontend::templates::TEMPLATES {
        for spelling in ["&#x27;", "&quot;", "&#x2f;"] {
            assert!(
                !source.contains(spelling),
                "{name} contains {spelling}, which markupsafe_compat would rewrite",
            );
        }
    }
}
