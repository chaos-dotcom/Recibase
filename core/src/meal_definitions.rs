//! `se.reciba.api.MealDefinitions`: the full meal list, i.e. the recipe corpus
//! plus the hand-written meal stubs.
//!
//! OWNER: workstream W5.

use crate::meal::MealStub;

/// `MealDefinitions.mealStubs` as a Scala `Set`.
pub fn meal_stubs() -> &'static [MealStub] {
    unimplemented!("meal_definitions::meal_stubs (workstream W5)")
}

/// The stub list in the order the Scala source declares it (used as the
/// insertion order for the `Set`).
pub fn declared_stubs() -> &'static [MealStub] {
    unimplemented!("meal_definitions::declared_stubs (workstream W5)")
}
