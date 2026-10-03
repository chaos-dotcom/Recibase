//! `se.reciba.api.submit.RecipeSource` - turn a submission into the Scala
//! source file that the pull request adds.

use crate::java::{take_utf16, utf16_len};
use crate::recipe_submission::{IngredientSubmission, RecipeSubmission};
use crate::scala_literal::ScalaLiteral;
use chrono::{Datelike, NaiveDate};
use recibase_core::permalink::strip_accents;
use recibase_core::recipe::RecipeDef;
use recibase_core::tag::TAGS;

/// `case class GeneratedRecipe`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRecipe {
    pub name: String,
    pub object_name: String,
    pub permalink: String,
    pub branch: String,
    pub path: String,
    pub source: String,
}

/// `sealed trait SubmitRejection`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitRejection {
    /// `final case class InvalidSubmission(message: String)`.
    InvalidSubmission(String),
    /// `final case class ConflictingSubmission(message: String)`.
    ConflictingSubmission(String),
}

impl SubmitRejection {
    pub fn message(&self) -> &str {
        match self {
            SubmitRejection::InvalidSubmission(message) => message,
            SubmitRejection::ConflictingSubmission(message) => message,
        }
    }
}

const NAME_LIMIT: usize = 80;
const TEXT_LIMIT: usize = 2000;
const LIST_LIMIT: usize = 80;
const TAG_LIMIT: usize = 40;

/// These identifiers are referenced by the generated file. A case object with
/// the same name would shadow them and fail to compile.
const RESERVED_IDENTIFIERS: &[&str] = &[
    "Recipe",
    "Tag",
    "Ingredient",
    "IngredientsBlock",
    "Permalink",
    "List",
    "Set",
    "None",
    "Some",
    "Option",
    "LocalDate",
];

const AUTOMATIC_TAGS: &[&str] = &["NeverEaten", "Popular", "Infrequent", "New"];

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanIngredient {
    name: String,
    quantity: Option<String>,
    prep: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanRecipe {
    name: String,
    source: Option<String>,
    description: Option<String>,
    notes: Vec<String>,
    tags: Vec<String>,
    ingredients: Vec<CleanIngredient>,
    method: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IdentifiedRecipe {
    clean: CleanRecipe,
    object_name: String,
    permalink: String,
}

pub struct RecipeSource;

impl RecipeSource {
    pub fn generate(
        submission: &RecipeSubmission,
        created_at: NaiveDate,
        existing: &[RecipeDef],
    ) -> Result<GeneratedRecipe, SubmitRejection> {
        let clean = RecipeSource::validate(submission)?;
        let identified = RecipeSource::identify(clean)?;
        RecipeSource::check_conflicts(&identified, existing)?;
        Ok(RecipeSource::render(&identified, created_at))
    }

    fn validate(submission: &RecipeSubmission) -> Result<CleanRecipe, SubmitRejection> {
        let name = submission.name.trim().to_string();
        let source = blank(submission.source.as_deref());
        let description = blank(submission.description.as_deref());
        let notes = trimmed_list(&submission.notes);
        let tags = distinct(&trimmed_list(&submission.tags));
        let method = trimmed_list(&submission.method);

        if submission.ingredients.len() > LIST_LIMIT {
            return invalid("Too many ingredients");
        }
        if submission.method.len() > LIST_LIMIT {
            return invalid("Too many method steps");
        }
        if submission.notes.len() > LIST_LIMIT {
            return invalid("Too many notes");
        }
        if submission.tags.len() > TAG_LIMIT {
            return invalid("Too many tags");
        }
        if name.is_empty() {
            return invalid("Name is required");
        }

        let checked_name = text("Name", &name, NAME_LIMIT, true)?;
        let checked_source = optional_text("Source", source.as_deref(), true)?;
        let checked_description = optional_text("Description", description.as_deref(), false)?;
        let checked_notes = list_text("A note", &notes)?;
        let checked_tags = parse_tags(&tags)?;
        let checked_ingredients = RecipeSource::ingredients(&submission.ingredients)?;
        if method.is_empty() {
            return invalid("Add at least one method step");
        }
        let checked_method = list_text("A method step", &method)?;

        Ok(CleanRecipe {
            name: checked_name,
            source: checked_source,
            description: checked_description,
            notes: checked_notes,
            tags: checked_tags,
            ingredients: checked_ingredients,
            method: checked_method,
        })
    }

    fn ingredients(
        submitted: &[IngredientSubmission],
    ) -> Result<Vec<CleanIngredient>, SubmitRejection> {
        let cleaned: Vec<CleanIngredient> = submitted
            .iter()
            .map(|ingredient| CleanIngredient {
                name: ingredient.name.trim().to_string(),
                quantity: blank(ingredient.quantity.as_deref()),
                prep: blank(ingredient.prep.as_deref()),
                notes: blank(ingredient.notes.as_deref()),
            })
            .filter(|ingredient| {
                !ingredient.name.is_empty()
                    || ingredient.quantity.is_some()
                    || ingredient.prep.is_some()
                    || ingredient.notes.is_some()
            })
            .collect();

        let mut result: Vec<CleanIngredient> = Vec::new();
        for ingredient in cleaned {
            if ingredient.name.is_empty() {
                return invalid("Each ingredient needs a name");
            }
            let name = text("An ingredient field", &ingredient.name, TEXT_LIMIT, true)?;
            let quantity = optional_text("An ingredient field", ingredient.quantity.as_deref(), true)?;
            let prep = optional_text("An ingredient field", ingredient.prep.as_deref(), true)?;
            let notes = optional_text("An ingredient field", ingredient.notes.as_deref(), true)?;
            result.push(CleanIngredient { name, quantity, prep, notes });
        }

        if result.is_empty() {
            invalid("Add at least one ingredient")
        } else {
            Ok(result)
        }
    }

    fn identify(clean: CleanRecipe) -> Result<IdentifiedRecipe, SubmitRejection> {
        let object_name: String = words(&clean.name).iter().map(|word| capitalise(word)).collect();
        let slug = permalink(&clean.name);
        if object_name.is_empty()
            || !is_object_name(&object_name)
            || RESERVED_IDENTIFIERS.contains(&object_name.as_str())
            || !is_permalink(&slug)
        {
            invalid("Recipe name cannot be turned into a Scala file name")
        } else {
            Ok(IdentifiedRecipe { clean, object_name, permalink: slug })
        }
    }

    fn check_conflicts(
        identified: &IdentifiedRecipe,
        existing: &[RecipeDef],
    ) -> Result<(), SubmitRejection> {
        let name = &identified.clean.name;
        if existing
            .iter()
            .any(|recipe| crate::java::equals_ignore_case(&recipe.name, name))
        {
            conflict(format!("A recipe named {} already exists", name))
        } else if existing
            .iter()
            .any(|recipe| recipe.permalink() == identified.permalink)
        {
            conflict(format!(
                "A recipe with permalink {} already exists",
                identified.permalink
            ))
        } else if existing
            .iter()
            .any(|recipe| recipe.object_name == identified.object_name)
        {
            conflict(format!(
                "A recipe file named {}.scala already exists",
                identified.object_name
            ))
        } else {
            Ok(())
        }
    }

    fn render(identified: &IdentifiedRecipe, created_at: NaiveDate) -> GeneratedRecipe {
        let clean = &identified.clean;
        let ingredient_lines: Vec<(String, bool)> =
            clean.ingredients.iter().map(ingredient_expr).collect();
        let needs_cats = clean.source.is_some()
            || clean.description.is_some()
            || ingredient_lines.iter().any(|(_, needs_cats)| *needs_cats);

        let mut imports: Vec<String> = Vec::new();
        if needs_cats {
            imports.push("import cats.syntax.option._".to_string());
        }
        imports.push(
            "import se.reciba.api.model.{Ingredient, IngredientsBlock, Permalink, Recipe, Tag}"
                .to_string(),
        );
        imports.push("import java.time.LocalDate".to_string());

        let mut metadata: Vec<String> = Vec::new();
        if let Some(value) = &clean.source {
            metadata.push(format!(
                "  override val source: Option[String] = {}.some",
                ScalaLiteral::quote(value)
            ));
        }
        if let Some(value) = &clean.description {
            metadata.push(format!(
                "  override val description: Option[String] = {}.some",
                ScalaLiteral::quote(value)
            ));
        }
        if !clean.notes.is_empty() {
            let notes: Vec<String> = clean.notes.iter().map(|note| ScalaLiteral::quote(note)).collect();
            metadata.extend(indented_list("override val notes: List[String] = List", &notes));
        }

        let tags = if clean.tags.is_empty() {
            "  val tags = Set.empty[Tag]".to_string()
        } else {
            let entries: Vec<String> = clean.tags.iter().map(|tag| format!("Tag.{}", tag)).collect();
            format!("  val tags = Set({})", entries.join(", "))
        };

        let mut lines: Vec<String> = vec![
            "package se.reciba.api.recipes".to_string(),
            String::new(),
        ];
        lines.extend(imports);
        lines.push(String::new());
        lines.push(format!("case object {} extends Recipe {{", identified.object_name));
        lines.push(format!("  val name = {}", ScalaLiteral::quote(&clean.name)));
        lines.push(format!(
            "  val createdAt = LocalDate.of({}, {}, {})",
            created_at.year(),
            created_at.month(),
            created_at.day()
        ));
        lines.push(format!(
            "  override val permalink: Permalink = Permalink({})",
            ScalaLiteral::quote(&identified.permalink)
        ));
        lines.push(String::new());
        if !metadata.is_empty() {
            lines.extend(metadata);
            lines.push(String::new());
        }
        lines.push(tags);
        lines.push(String::new());
        lines.extend(indented_list(
            "val ingredientsBlocks = IngredientsBlock.simple",
            &ingredient_lines
                .iter()
                .map(|(expression, _)| expression.clone())
                .collect::<Vec<String>>(),
        ));
        lines.push(String::new());
        lines.extend(indented_list(
            "val method = List",
            &clean
                .method
                .iter()
                .map(|step| ScalaLiteral::quote(step))
                .collect::<Vec<String>>(),
        ));
        lines.push("}".to_string());

        GeneratedRecipe {
            name: clean.name.clone(),
            object_name: identified.object_name.clone(),
            permalink: identified.permalink.clone(),
            branch: format!("recipe/{}", identified.permalink),
            path: format!(
                "src/main/scala/se/reciba/api/recibase/recipes/{}.scala",
                identified.object_name
            ),
            source: format!("{}\n", lines.join("\n")),
        }
    }
}

/// `Ingredient(...)` for one cleaned ingredient, and whether the expression
/// needs `cats.syntax.option._`.
fn ingredient_expr(ingredient: &CleanIngredient) -> (String, bool) {
    let name = ScalaLiteral::quote(&ingredient.name);
    match (&ingredient.quantity, &ingredient.prep, &ingredient.notes) {
        (None, None, None) => (format!("Ingredient({})", name), false),
        (Some(quantity), None, None) => (
            format!("Ingredient({}, {})", name, ScalaLiteral::quote(quantity)),
            false,
        ),
        (Some(quantity), Some(prep), None) => (
            format!(
                "Ingredient({}, {}, {})",
                name,
                ScalaLiteral::quote(quantity),
                ScalaLiteral::quote(prep)
            ),
            false,
        ),
        (Some(quantity), Some(prep), Some(notes)) => (
            format!(
                "Ingredient({}, {}, {}, {})",
                name,
                ScalaLiteral::quote(quantity),
                ScalaLiteral::quote(prep),
                ScalaLiteral::quote(notes)
            ),
            false,
        ),
        (quantity, prep, notes) => (
            format!(
                "Ingredient({}, {}, {}, {})",
                name,
                opt(quantity.as_deref()),
                opt(prep.as_deref()),
                opt(notes.as_deref())
            ),
            true,
        ),
    }
}

fn opt(value: Option<&str>) -> String {
    match value {
        None => "None".to_string(),
        Some(text) => format!("{}.some", ScalaLiteral::quote(text)),
    }
}

fn indented_list(prefix: &str, items: &[String]) -> Vec<String> {
    let mut out = vec![format!("  {}(", prefix)];
    for (index, item) in items.iter().enumerate() {
        let comma = if index == items.len() - 1 { "" } else { "," };
        out.push(format!("    {}{}", item, comma));
    }
    out.push("  )".to_string());
    out
}

fn words(name: &str) -> Vec<String> {
    strip_accents(name)
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect()
}

fn capitalise(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let rest: String = chars.as_str().to_lowercase();
            format!("{}{}", first.to_ascii_uppercase(), rest)
        }
    }
}

fn permalink(name: &str) -> String {
    let lowered = strip_accents(name).to_lowercase();
    let mut out = String::new();
    let mut pending_dash = false;
    for character in lowered.chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(character);
        } else {
            pending_dash = true;
        }
    }
    out
}

fn is_object_name(name: &str) -> bool {
    let mut characters = name.chars();
    match characters.next() {
        Some(first) if first.is_ascii_uppercase() => {
            characters.all(|character| character.is_ascii_alphanumeric())
        }
        _ => false,
    }
}

fn is_permalink(slug: &str) -> bool {
    !slug.is_empty()
        && slug.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

fn blank(value: Option<&str>) -> Option<String> {
    value
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

fn trimmed_list(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

fn distinct(values: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for value in values {
        if !out.contains(value) {
            out.push(value.clone());
        }
    }
    out
}

fn parse_tags(tags: &[String]) -> Result<Vec<String>, SubmitRejection> {
    let mut result: Vec<String> = Vec::new();
    for tag in tags {
        if unsupported(tag, true) {
            return invalid("A tag contains unsupported characters");
        }
        if AUTOMATIC_TAGS.contains(&tag.as_str()) {
            return invalid(format!("Tag {} is assigned automatically", tag));
        }
        match tag_object_name(tag) {
            Some(object_name) => result.push(object_name.to_string()),
            None => {
                let shown = if utf16_len(tag) > 40 {
                    format!("{}...", take_utf16(tag, 40))
                } else {
                    tag.clone()
                };
                return invalid(format!("Unknown tag: {}", shown));
            }
        }
    }
    Ok(result)
}

/// `tagsByObjectName`: `getClass.getSimpleName.stripSuffix("$")` for every
/// `Tag` value, mapped to itself.
fn tag_object_name(tag: &str) -> Option<&'static str> {
    TAGS.iter()
        .map(|value| value.object_name())
        .find(|object_name| *object_name == tag)
}

fn optional_text(
    label: &str,
    value: Option<&str>,
    single_line: bool,
) -> Result<Option<String>, SubmitRejection> {
    match value {
        None => Ok(None),
        Some(text_value) => text(label, text_value, TEXT_LIMIT, single_line).map(Some),
    }
}

fn list_text(label: &str, values: &[String]) -> Result<Vec<String>, SubmitRejection> {
    let mut result = Vec::with_capacity(values.len());
    for value in values {
        result.push(text(label, value, TEXT_LIMIT, false)?);
    }
    Ok(result)
}

fn text(
    label: &str,
    value: &str,
    max: usize,
    single_line: bool,
) -> Result<String, SubmitRejection> {
    if unsupported(value, single_line) {
        invalid(format!("{} contains unsupported characters", label))
    } else if utf16_len(value) > max {
        invalid(format!("{} is too long", label))
    } else {
        Ok(value.to_string())
    }
}

fn unsupported(value: &str, single_line: bool) -> bool {
    for code_point in value.chars() {
        let blocked = code_point == '\u{2028}'
            || code_point == '\u{2029}'
            || (is_iso_control(code_point)
                && code_point != '\n'
                && code_point != '\r'
                && code_point != '\t')
            || (single_line
                && (code_point == '\n' || code_point == '\r' || code_point == '\t'));
        if blocked {
            return true;
        }
    }
    false
}

/// `Character.isISOControl`.
fn is_iso_control(code_point: char) -> bool {
    let value = code_point as u32;
    value <= 0x1f || (0x7f..=0x9f).contains(&value)
}

fn invalid<T>(message: impl Into<String>) -> Result<T, SubmitRejection> {
    Err(SubmitRejection::InvalidSubmission(message.into()))
}

fn conflict<T>(message: String) -> Result<T, SubmitRejection> {
    Err(SubmitRejection::ConflictingSubmission(message))
}
