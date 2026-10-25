//! Navigation integration for the imported Two Stories with Interiors house.
//! Visible architecture comes from the GLB. The previous Autumn House collision
//! shell is intentionally disabled because its bounds do not match this model.
use crate::scene::SceneSpec;

pub(crate) fn append(_s: &mut SceneSpec) {
    // House collision will be rebuilt from the new model's measured bounds.
    // Keeping the Autumn House wall volumes here would create invisible blockers.
}
