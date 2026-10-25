//! Tags, mirroring `se.reciba.api.model.Tag` on upstream master.
//!
//! A tag has a display `entry_name`, a `category`, an optional `description`,
//! and an optional `superset`: the next, looser tag in its chain. Following the
//! chain gives the tags the recipe implies - a `Vegan` recipe is also
//! `Vegan-ish`, `Vegetarian`, `Vegetarian-ish` and `Pescatarian`.
//!
//! `entry_name`, `category` and `description` are the JSON forms; `object_name`
//! is the Scala identifier, used by the recipe generator.

/// `TagCategory`. The order here is the declaration order, which is also the
/// order `/tags` reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TagCategory {
    Meal,
    Diet,
    Stephani,
    Vibe,
    Making,
    Practical,
    Usage,
}

impl TagCategory {
    /// The JSON string form (`entryName`).
    pub fn entry_name(self) -> &'static str {
        match self {
            TagCategory::Meal => "meal",
            TagCategory::Diet => "diet",
            TagCategory::Stephani => "stephani",
            TagCategory::Vibe => "vibe",
            TagCategory::Making => "making",
            TagCategory::Practical => "practical",
            TagCategory::Usage => "usage",
        }
    }
}

/// Declaration order, as `TagCategory.values` (enumeratum `findValues`).
pub const TAG_CATEGORIES: &[TagCategory] = &[
    TagCategory::Meal,
    TagCategory::Diet,
    TagCategory::Stephani,
    TagCategory::Vibe,
    TagCategory::Making,
    TagCategory::Practical,
    TagCategory::Usage,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tag {
    Christmas,
    Pudding,
    Lunch,
    Baking,
    IncludeInMealPlanner,
    ExcludeFromMealPlanner,
    Soup,
    Vegan,
    VeganIsh,
    Vegetarian,
    VegetarianIsh,
    Pescatarian,
    GlutenFree,
    StephaniUnhealthy,
    StephaniIsh,
    Stephani,
    ColdWeather,
    HotWeather,
    Stodge,
    Spicy,
    Slow,
    Quick,
    Scales,
    HighEffort,
    LowEffort,
    Freezes,
    BetterNextDay,
    AI,
    NeverEaten,
    Popular,
    Infrequent,
    New,
}

/// Declaration order, as `Tag.values` (enumeratum `findValues`). This is the
/// order `/tags` reports.
pub const TAGS: &[Tag] = &[
    Tag::Christmas,
    Tag::Pudding,
    Tag::Lunch,
    Tag::Baking,
    Tag::IncludeInMealPlanner,
    Tag::ExcludeFromMealPlanner,
    Tag::Soup,
    Tag::Vegan,
    Tag::VeganIsh,
    Tag::Vegetarian,
    Tag::VegetarianIsh,
    Tag::Pescatarian,
    Tag::GlutenFree,
    Tag::StephaniUnhealthy,
    Tag::StephaniIsh,
    Tag::Stephani,
    Tag::ColdWeather,
    Tag::HotWeather,
    Tag::Stodge,
    Tag::Spicy,
    Tag::Slow,
    Tag::Quick,
    Tag::Scales,
    Tag::HighEffort,
    Tag::LowEffort,
    Tag::Freezes,
    Tag::BetterNextDay,
    Tag::AI,
    Tag::NeverEaten,
    Tag::Popular,
    Tag::Infrequent,
    Tag::New,
];

impl Tag {
    /// The JSON string form (`entryName`).
    pub fn entry_name(self) -> &'static str {
        match self {
            Tag::Christmas => "Christmas",
            Tag::Pudding => "Pudding",
            Tag::Lunch => "Lunch",
            Tag::Baking => "Baking",
            Tag::IncludeInMealPlanner => "Include in Meal Planner",
            Tag::ExcludeFromMealPlanner => "Exclude from Meal Planner",
            Tag::Soup => "Soup",
            Tag::Vegan => "Vegan",
            Tag::VeganIsh => "Vegan-ish",
            Tag::Vegetarian => "Vegetarian",
            Tag::VegetarianIsh => "Vegetarian-ish",
            Tag::Pescatarian => "Pescatarian",
            Tag::GlutenFree => "Gluten-Free",
            Tag::StephaniUnhealthy => "StephaniUnhealthy",
            Tag::StephaniIsh => "Stephani-ish",
            Tag::Stephani => "Stephani",
            Tag::ColdWeather => "Cold Weather",
            Tag::HotWeather => "Hot Weather",
            Tag::Stodge => "Stodge",
            Tag::Spicy => "Spicy",
            Tag::Slow => "Slow",
            Tag::Quick => "Quick",
            Tag::Scales => "Scales",
            Tag::HighEffort => "High Effort",
            Tag::LowEffort => "Low Effort",
            Tag::Freezes => "Freezes",
            Tag::BetterNextDay => "Better Next Day",
            Tag::AI => "AI",
            Tag::NeverEaten => "Never Eaten",
            Tag::Popular => "Popular",
            Tag::Infrequent => "Infrequent",
            Tag::New => "New",
        }
    }

    pub fn category(self) -> TagCategory {
        match self {
            Tag::Christmas
            | Tag::Pudding
            | Tag::Lunch
            | Tag::Baking
            | Tag::IncludeInMealPlanner
            | Tag::ExcludeFromMealPlanner
            | Tag::Soup => TagCategory::Meal,
            Tag::Vegan
            | Tag::VeganIsh
            | Tag::Vegetarian
            | Tag::VegetarianIsh
            | Tag::Pescatarian
            | Tag::GlutenFree => TagCategory::Diet,
            Tag::StephaniUnhealthy | Tag::StephaniIsh | Tag::Stephani => TagCategory::Stephani,
            Tag::ColdWeather | Tag::HotWeather | Tag::Stodge | Tag::Spicy => TagCategory::Vibe,
            Tag::Slow | Tag::Quick | Tag::Scales | Tag::HighEffort | Tag::LowEffort => {
                TagCategory::Making
            }
            Tag::Freezes | Tag::BetterNextDay | Tag::AI => TagCategory::Practical,
            Tag::NeverEaten | Tag::Popular | Tag::Infrequent | Tag::New => TagCategory::Usage,
        }
    }

    /// `description`. Four tags carry none, which is JSON `null`.
    pub fn description(self) -> Option<&'static str> {
        match self {
            Tag::Lunch => Some("Typically a lunch dish, sometimes eaten for dinner"),
            Tag::Baking => Some("A baked good like a cake"),
            Tag::IncludeInMealPlanner => {
                Some("Ensures recipe is included in meal planner. Only use if necessary.")
            }
            Tag::ExcludeFromMealPlanner => {
                Some("Ensures recipe is excluded from meal planner. Only use if necessary.")
            }
            Tag::Vegan => Some("No animal products"),
            Tag::VeganIsh => Some("Can make vegan with minimal changes"),
            Tag::Vegetarian => Some("No meat or fish"),
            Tag::VegetarianIsh => Some("Can make vegetarian with minimal changes"),
            Tag::Pescatarian => Some("Contains fish but not meat"),
            Tag::StephaniUnhealthy => {
                Some("Contains no Stephani allergens but low on nutrients")
            }
            Tag::StephaniIsh => Some("Can make Stephani compatible with minimal changes"),
            Tag::Stephani => Some("Contains no Stephani allergens"),
            Tag::ColdWeather => Some("Suited to winter"),
            Tag::HotWeather => Some("Suited to hot summer"),
            Tag::Stodge => Some("Hearty and filling but not healthy"),
            Tag::Spicy => Some("Hot, rather than just aromatic"),
            Tag::Slow => Some("Takes a long time to cook, including baking/simmering"),
            Tag::Quick => Some("Under 25 minutes from start to serving"),
            Tag::Scales => Some("Easy to scale up to many servings"),
            Tag::HighEffort => Some("Takes a lot of work"),
            Tag::LowEffort => Some("Only a few steps, no major preparation"),
            Tag::Freezes => Some("Keeps well in the freezer"),
            Tag::BetterNextDay => Some("Benefits from a cool/reheat cycle"),
            Tag::AI => Some("The recipe text was drafted with AI, double check for slop"),
            Tag::NeverEaten => Some("Never eaten for dinner"),
            Tag::Popular => Some("Eaten for dinner at least five times"),
            Tag::Infrequent => Some("Eaten for dinner once or twice"),
            Tag::New => Some("Added in the last 12 months"),
            Tag::Christmas | Tag::Pudding | Tag::Soup | Tag::GlutenFree => None,
        }
    }

    /// `superset`: the next, looser tag in this tag's chain.
    ///
    /// Note the `Stephani` chain runs the other way round from the dietary one:
    /// `Stephani` (no allergens) is the strictest and implies `Stephani-ish`,
    /// which implies `StephaniUnhealthy`.
    pub fn superset(self) -> Option<Tag> {
        match self {
            Tag::Vegan => Some(Tag::VeganIsh),
            Tag::VeganIsh => Some(Tag::Vegetarian),
            Tag::Vegetarian => Some(Tag::VegetarianIsh),
            Tag::VegetarianIsh => Some(Tag::Pescatarian),
            Tag::Stephani => Some(Tag::StephaniIsh),
            Tag::StephaniIsh => Some(Tag::StephaniUnhealthy),
            _ => None,
        }
    }

    /// `impliedTags`: the transitive supersets, excluding self, **in chain
    /// order** (nearest first).
    ///
    /// Upstream walks the chain breadth-first from `superset`, so `Vegan` gives
    /// `Vegan-ish, Vegetarian, Vegetarian-ish, Pescatarian`.
    pub fn implied_tags(self) -> Vec<Tag> {
        let mut out = Vec::new();
        let mut cur = self.superset();
        while let Some(next) = cur {
            out.push(next);
            cur = next.superset();
        }
        out
    }

    /// The Scala object identifier, e.g. `VeganIsh`, `GlutenFree`.
    pub fn object_name(self) -> &'static str {
        match self {
            Tag::VeganIsh => "VeganIsh",
            Tag::VegetarianIsh => "VegetarianIsh",
            Tag::GlutenFree => "GlutenFree",
            Tag::StephaniUnhealthy => "StephaniUnhealthy",
            Tag::StephaniIsh => "StephaniIsh",
            Tag::ColdWeather => "ColdWeather",
            Tag::HotWeather => "HotWeather",
            Tag::HighEffort => "HighEffort",
            Tag::LowEffort => "LowEffort",
            Tag::BetterNextDay => "BetterNextDay",
            Tag::IncludeInMealPlanner => "IncludeInMealPlanner",
            Tag::ExcludeFromMealPlanner => "ExcludeFromMealPlanner",
            Tag::NeverEaten => "NeverEaten",
            _ => self.entry_name(),
        }
    }

    pub fn from_object_name(name: &str) -> Option<Tag> {
        TAGS.iter().copied().find(|t| t.object_name() == name)
    }

    pub fn from_entry_name(name: &str) -> Option<Tag> {
        TAGS.iter().copied().find(|t| t.entry_name() == name)
    }

    /// `Tag.excludeFromMealPlannerTags`.
    pub fn exclude_from_meal_planner_tags() -> Vec<Tag> {
        vec![Tag::Pudding, Tag::Baking, Tag::ExcludeFromMealPlanner]
    }

    /// `Meal.includeinMealPlanner`: an explicit include wins, otherwise the
    /// meal is included unless it carries an exclude tag.
    pub fn include_in_meal_planner(tags: &[Tag]) -> bool {
        if tags.contains(&Tag::IncludeInMealPlanner) {
            true
        } else {
            let excluded = Self::exclude_from_meal_planner_tags();
            !tags.iter().any(|tag| excluded.contains(tag))
        }
    }
}
