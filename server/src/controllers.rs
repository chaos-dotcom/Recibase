//! `RecipeController`, `MealsController` and `MetaController`.

use chrono::{Datelike, Duration, Local, Months, NaiveDate};
use recibase_core::json::to_string;
use recibase_core::meal::{MealStub, MealStubWithUsageData};
use recibase_core::misc::{docs_json, Manifest, MenuEntry};
use recibase_core::recipe::RecipeDef;
use recibase_core::scala_hash;
use recibase_core::tag::Tag;
use serde_json::Value;
use std::collections::HashMap;

pub struct Usage {
    pub entries: Vec<recibase_core::usage::MealLogEntry>,
    pub today: NaiveDate,
}

impl Usage {
    pub fn meal_notes(&self) -> HashMap<String, Vec<recibase_core::meal::DatedNote>> {
        recibase_core::usage::notes(&self.entries)
    }
    pub fn totals(&self) -> HashMap<String, i64> {
        recibase_core::usage::totals(&self.entries)
    }
    pub fn last_eaten(&self) -> HashMap<String, NaiveDate> {
        recibase_core::usage::last_eaten(&self.entries)
    }
    pub fn featured(&self) -> HashMap<String, NaiveDate> {
        recibase_core::usage::featured_meals(&self.entries)
    }
}

/// `RecipeController.listRecipes`.
pub fn list_recipes(has_ingredient: Option<&str>) -> Vec<MenuEntry> {
    let mut entries: Vec<MenuEntry> = recibase_core::recipes::recipes()
        .iter()
        .filter(|recipe| match has_ingredient {
            None => true,
            Some(ingredient) => recipe.has_ingredient(ingredient),
        })
        .map(|recipe| MenuEntry::new(&recipe.name, &recipe.permalink()))
        .collect();
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    entries
}

pub fn recipes_json(has_ingredient: Option<&str>) -> Value {
    Value::Array(list_recipes(has_ingredient).iter().map(|e| e.to_json()).collect())
}

/// `RecipeController.routing`: permalink -> recipe, built once at start-up as
/// the Scala's `private val routing` is.
fn routing() -> &'static HashMap<String, &'static RecipeDef> {
    static ROUTING: std::sync::LazyLock<HashMap<String, &'static RecipeDef>> =
        std::sync::LazyLock::new(|| {
            recibase_core::recipes::recipes()
                .iter()
                .map(|recipe| (recipe.permalink(), recipe))
                .collect()
        });
    &ROUTING
}

/// `RecipeController.getRecipe`.
pub fn recipe_json(permalink: &str, usage: &Usage) -> Option<Value> {
    routing().get(permalink).map(|recipe| {
        let notes = usage.meal_notes();
        let dated = notes.get(&recipe.name).cloned().unwrap_or_default();
        recipe.to_json_with_usage(&dated)
    })
}

/// `MealsController.mealStubsWithUsageData`.
pub fn meals_with_usage(usage: &Usage) -> Vec<MealStubWithUsageData> {
    let totals = usage.totals();
    let last_eaten = usage.last_eaten();
    let notes = usage.meal_notes();
    let featured = usage.featured();

    let mut out = Vec::new();
    let stubs: Vec<MealStub> = recibase_core::meal_definitions::meal_stubs().to_vec();
    for meal in stubs.iter() {
        let times_eaten = totals.get(&meal.name).copied().unwrap_or(0);
        let extra_frequency: Vec<Tag> = if !meal.is_dinner() {
            Vec::new()
        } else if times_eaten == 0 {
            vec![Tag::NeverEaten]
        } else if times_eaten <= 2 {
            vec![Tag::Infrequent]
        } else if times_eaten >= 5 {
            vec![Tag::Popular]
        } else {
            Vec::new()
        };
        let extra_recency: Vec<Tag> = match meal.created_at {
            Some(created) => match usage.today.checked_sub_months(Months::new(12)) {
                Some(cutoff) if created > cutoff => vec![Tag::New],
                _ => Vec::new(),
            },
            _ => Vec::new(),
        };
        let mut tags = meal.tags.clone();
        tags.extend(extra_frequency);
        tags.extend(extra_recency);
        out.push(MealStubWithUsageData {
            name: meal.name.clone(),
            tags,
            source: meal.source.clone(),
            dated_notes: notes.get(&meal.name).cloned().unwrap_or_default(),
            last_eaten: last_eaten.get(&meal.name).copied(),
            times_eaten,
            featured: featured.get(&meal.name).copied(),
        });
    }
    out
}

pub fn meals_json(usage: &Usage) -> Value {
    let meals = meals_with_usage(usage);
    let order = scala_hash::set_order(&meals);
    Value::Array(order.iter().map(|&i| meals[i].to_json()).collect())
}

/// `MealsController.mealNames`.
pub fn meal_names() -> String {
    let mut names: Vec<String> = recibase_core::meal_definitions::meal_stubs()
        .iter()
        .filter(|meal| meal.is_dinner())
        .map(|meal| meal.name.clone())
        .collect();
    names.sort();
    names.join("\n")
}

pub fn manifest_json(env: &dyn Fn(&str) -> Option<String>) -> Value {
    Manifest { version: Manifest::deployed_version(env) }.to_json()
}

pub fn docs() -> Value {
    docs_json()
}

/// The `New` tag depends on the day the server runs, so the comparison harness
/// can pin it. Defaults to the local date, exactly like the Scala.
pub fn today(env: &dyn Fn(&str) -> Option<String>) -> NaiveDate {
    match env("RECIBASE_TODAY") {
        Some(value) => NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap_or_else(|_| Local::now().date_naive()),
        None => Local::now().date_naive(),
    }
}

/// `LocalDate.now(ZoneId.of("Europe/London"))` for the submission route.
pub fn london_today() -> NaiveDate {
    use chrono::Utc;
    let london = chrono::FixedOffset::east_opt(0).unwrap();
    let _ = london;
    let now = Utc::now() + Duration::hours(1);
    NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap_or_else(|| Local::now().date_naive())
}

pub fn _unused(value: &Value) -> String {
    to_string(value)
}
