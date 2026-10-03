//! `RecipeDef::revision`: a stable content digest used to tell \"the same
//! recipe\" from \"the same name, a different recipe\" across deployments.

use recibase_core::recipes;

#[test]
fn revision_is_stable_and_content_sensitive() {
    let recipe = &recipes::recipes()[0];
    let digest = recipe.revision();

    assert_eq!(digest, recipe.revision(), "stable across calls");
    assert_eq!(digest.len(), 16);
    assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));

    let mut changed = recipe.clone();
    changed.name.push_str(" (changed)");
    assert_ne!(digest, changed.revision());
}

/// `edit` is built from the object name and points at each deployment's own
/// repository, so it must not change the digest - otherwise two servers could
/// never agree that they hold the same recipe.
#[test]
fn revision_ignores_the_deployment_specific_edit_link() {
    let recipe = &recipes::recipes()[0];
    let mut renamed = recipe.clone();
    renamed.object_name.push('X');
    assert_eq!(recipe.revision(), renamed.revision());
}
