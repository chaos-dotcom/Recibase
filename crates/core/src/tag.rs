//! Tags, mirroring `se.reciba.api.model.Tag`.
//!
//! `entry_name` is the JSON string form (enumeratum `entryName`); `object_name`
//! is the Scala identifier (`getClass.getSimpleName.stripSuffix("$")`) used by
//! the recipe generator.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tag {
    Christmas,
    Pudding,
    Lunch,
    Baking,
    NonMeal,
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

/// Declaration order, as `Tag.values` (enumeratum `findValues`).
pub const TAGS: &[Tag] = &[
    Tag::Christmas,
    Tag::Pudding,
    Tag::Lunch,
    Tag::Baking,
    Tag::NonMeal,
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
            Tag::NonMeal => "Not a Meal",
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

    /// The Scala object identifier, e.g. `NonMeal`, `VeganIsh`.
    pub fn object_name(self) -> &'static str {
        match self {
            Tag::NonMeal => "NonMeal",
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

    pub fn parent(self) -> Option<Tag> {
        match self {
            Tag::Vegan => Some(Tag::VeganIsh),
            Tag::VeganIsh => Some(Tag::Vegetarian),
            Tag::Vegetarian => Some(Tag::VegetarianIsh),
            Tag::VegetarianIsh => Some(Tag::Pescatarian),
            Tag::StephaniUnhealthy => Some(Tag::StephaniIsh),
            Tag::StephaniIsh => Some(Tag::Stephani),
            _ => None,
        }
    }

    /// `allParentTags` - the transitive parents, excluding self.
    ///
    /// Scala builds the result as `parent.allParentTags + parent`, so the
    /// outermost ancestor comes first: for `Vegan` that is
    /// `Pescatarian, VegetarianIsh, Vegetarian, VeganIsh`. The order is
    /// observable: `Set.flatMap` keeps insertion order for a result of four or
    /// fewer elements.
    pub fn all_parent_tags(self) -> Vec<Tag> {
        let mut out = Vec::new();
        let mut cur = self.parent();
        while let Some(p) = cur {
            out.push(p);
            cur = p.parent();
        }
        out.reverse();
        out
    }

    pub fn non_dinner_tags() -> Vec<Tag> {
        vec![Tag::Pudding, Tag::Lunch, Tag::Baking, Tag::NonMeal]
    }
}
