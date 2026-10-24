//! Rendering recipe prose - method steps and notes - from Markdown to HTML.
//!
//! The API serves recipe prose as Markdown, so the same recipe reads the same
//! way on every deployment and a peer's recipe needs no conversion on the way
//! in. Rendering happens here, in the frontend, rather than in the server: the
//! wire format stays plain text, and what the reader sees is a presentation
//! decision.
//!
//! Only the prose is Markdown. An ingredient stays a structured object
//! (`name`/`quantity`/`prep`/`notes`), because the scaler parses `quantity`,
//! `hasIngredient` matches `name`, and the clipboard merges duplicates across
//! blocks - all of which need the fields, not a sentence.
//!
//! Two things the renderer does deliberately:
//!
//! * Raw HTML is passed through. The prose already contains trusted HTML (the
//!   recipe sources are reviewed before they ship), and the scaler's emphasis
//!   of method-step quantities is a `<span class="scaled">` that must survive
//!   the render.
//! * A prose field is a *fragment*, not a document, so a single paragraph is
//!   unwrapped. A method step is already inside an `<li>`, and a note inside a
//!   `<p>`: emitting `<p>` inside either would be invalid HTML.

use markdown::mdast::Node;
use markdown::{CompileOptions, Options, to_html_with_options, to_mdast};

/// The options every recipe field is rendered with.
///
/// `allow_dangerous_html` keeps the trusted markup in the recipe sources (and
/// the scaler's `<span class="scaled">`) intact instead of escaping it into
/// visible text.
fn options() -> Options {
    Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            ..CompileOptions::default()
        },
        ..Options::default()
    }
}

/// Renders one Markdown fragment as inline HTML, suitable for placing inside an
/// existing block element such as `<li>` or `<p>`.
///
/// A fragment is usually one paragraph, and is returned without its `<p>`
/// wrapper because the element around it already supplies one. A fragment
/// written as several paragraphs has them unwrapped and joined, so a note can
/// never nest a `<p>` inside the `<p>` the template already emits.
///
/// A fragment that is meant as one step or one note should be written as prose.
/// A line beginning `- `, `# ` or `1. ` is block syntax: it is real structure,
/// so it is rendered as a list or a heading, nested inside the wrapping element.
#[must_use]
pub fn to_inline_html(source: &str) -> String {
    // `to_mdast` cannot fail without MDX, which is off.
    let Ok(Node::Root(root)) = to_mdast(source, &options().parse) else {
        return render(source);
    };
    // Only paragraphs can be unwrapped safely. Anything else - a list, a quote,
    // a heading - is structure the author asked for, and is left alone.
    if root
        .children
        .iter()
        .all(|child| matches!(child, Node::Paragraph(_)))
    {
        return root
            .children
            .iter()
            .map(|child| match child {
                Node::Paragraph(paragraph) => render_inline_children(&paragraph.children, source),
                _ => String::new(),
            })
            .collect::<Vec<String>>()
            .join(" ");
    }
    render(source)
}

/// Renders one Markdown fragment as HTML, keeping the block structure the
/// Markdown describes.
#[must_use]
pub fn to_block_html(source: &str) -> String {
    render(source)
}

fn render(source: &str) -> String {
    // `to_html_with_options` cannot fail without MDX, which is off.
    to_html_with_options(source, &options()).unwrap_or_else(|_| crate::markupsafe::escape(source))
}

/// Renders a paragraph's children as inline HTML.
///
/// The source text the node came from is used to recover the original inline
/// Markdown, so the crate itself does the inline work: emphasis, links, inline
/// HTML and character references all behave exactly as they do in a document.
fn render_inline_children(children: &[Node], source: &str) -> String {
    let html = render(&inline_source(children, source));
    unwrap_single_paragraph(&html).unwrap_or(html)
}

/// The Markdown of a paragraph's inline content, taken back out of `source`.
///
/// Each child's position is a source range, so the paragraph's slice is exactly
/// the text the author wrote, with its inline markup intact.
fn inline_source(children: &[Node], source: &str) -> String {
    let (Some(first), Some(last)) = (children.first(), children.last()) else {
        return String::new();
    };
    let (Some(start), Some(end)) = (first.position(), last.position()) else {
        return source.to_string();
    };
    source
        .get(start.start.offset..end.end.offset)
        .unwrap_or(source)
        .to_string()
}

/// `"<p>text</p>"` -> `"text"`, for a single top-level paragraph and nothing
/// else. Returns `None` when the HTML is anything more than that, so a list or
/// a run of paragraphs keeps the markup that gives it its shape.
fn unwrap_single_paragraph(html: &str) -> Option<String> {
    const OPEN: &str = "<p>";
    const CLOSE: &str = "</p>";
    let trimmed = html.trim();
    let inner = trimmed.strip_prefix(OPEN)?.strip_suffix(CLOSE)?;
    // A paragraph cannot contain another one: if it does, this is not the
    // single-paragraph shape it looks like.
    if inner.contains(OPEN) {
        return None;
    }
    Some(inner.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_prose_is_unchanged_by_the_round_trip() {
        assert_eq!(to_block_html("Brown the mince."), "<p>Brown the mince.</p>");
        assert_eq!(to_inline_html("Brown the mince."), "Brown the mince.");
    }

    #[test]
    fn emphasis_becomes_html() {
        assert_eq!(
            to_inline_html("Simmer **gently** for 10 mins."),
            "Simmer <strong>gently</strong> for 10 mins."
        );
        assert_eq!(
            to_inline_html("Chop the *onion*."),
            "Chop the <em>onion</em>."
        );
    }

    #[test]
    fn a_markdown_link_becomes_the_same_anchor_as_the_html_it_replaces() {
        let rendered =
            to_inline_html("[An arborio rice/water ratio calculator](https://t.sci1.uk/x/)");
        assert_eq!(
            rendered,
            "<a href=\"https://t.sci1.uk/x/\">An arborio rice/water ratio calculator</a>"
        );
    }

    #[test]
    fn existing_trusted_html_is_passed_through_rather_than_escaped() {
        // The scaler wraps a scaled quantity like this; it has to survive.
        assert_eq!(
            to_inline_html("Add <span class=\"scaled\">200g</span> flour."),
            "Add <span class=\"scaled\">200g</span> flour."
        );
    }

    #[test]
    fn a_lone_asterisk_in_prose_is_not_treated_as_emphasis() {
        assert_eq!(to_inline_html("Use 1 * 2 = 2."), "Use 1 * 2 = 2.");
    }

    #[test]
    fn a_single_paragraph_is_unwrapped_for_inline_contexts_only() {
        assert_eq!(
            to_inline_html("Mix in the goats cheese."),
            "Mix in the goats cheese."
        );
        assert_eq!(
            to_block_html("Mix in the goats cheese."),
            "<p>Mix in the goats cheese.</p>"
        );
    }

    #[test]
    fn a_list_keeps_its_block_markup() {
        let rendered = to_inline_html("- one\n- two");
        assert!(rendered.starts_with("<ul>"), "{rendered}");
        assert!(rendered.contains("<li>one</li>"), "{rendered}");
    }

    /// A step or note is written as prose. Block syntax would nest structure
    /// inside the wrapping `<li>`/`<p>`, so this pins what that looks like.
    #[test]
    fn block_syntax_in_a_fragment_is_rendered_as_structure() {
        assert_eq!(
            to_inline_html("- add the stock"),
            "<ul>\n<li>add the stock</li>\n</ul>"
        );
        assert_eq!(
            to_inline_html("# a hash-led step"),
            "<h1>a hash-led step</h1>"
        );
    }

    #[test]
    fn a_block_marker_mid_sentence_is_plain_prose() {
        assert_eq!(to_inline_html("Stir 1. Then rest."), "Stir 1. Then rest.");
    }

    #[test]
    fn several_paragraphs_are_unwrapped_so_a_note_never_nests_a_paragraph() {
        // `recipe.html` puts a lone note inside its own `<p>`, so the renderer
        // must not hand it another one.
        assert_eq!(
            to_inline_html("First paragraph.\n\nSecond paragraph."),
            "First paragraph. Second paragraph."
        );
        assert_eq!(
            to_inline_html(
                "You can substitute Shiitake.\nPots of dried forest mushrooms also work."
            ),
            "You can substitute Shiitake.\nPots of dried forest mushrooms also work."
        );
    }

    #[test]
    fn empty_input_renders_empty() {
        assert_eq!(to_inline_html(""), "");
        assert_eq!(to_block_html(""), "");
    }
}
