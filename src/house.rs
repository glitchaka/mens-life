//! Imported Autumn House integration.
//! Visual architecture is converted from the supplied `House_scene_01.fbx`;
//! this module owns only asset materialization and structural collision proxies.
use crate::scene::SceneSpec;
use serde_json::json;

pub const VISUAL_ASSET: &str = "house/autumn_house.gltf#Scene0";

/// Reassembles the compact FBX-derived glTF into Bevy's runtime asset root.
/// The source is split only to keep repository text writes manageable; at
/// runtime Bevy sees a normal single glTF file with embedded vertex data.
pub fn prepare_asset_root() -> String {
    const EXPECTED_BYTES: usize = 60_241;
    let root = std::env::temp_dir().join("mens-life-assets-autumn-house-v2");
    let folder = root.join("house");
    let target = folder.join("autumn_house.gltf");
    let valid = std::fs::metadata(&target)
        .is_ok_and(|metadata| metadata.len() == EXPECTED_BYTES as u64);
    if !valid {
        std::fs::create_dir_all(&folder).expect("cannot create Autumn House asset cache");
        let chunks = [
            include_str!("../assets/house/autumn_house.gltf.part00"),
            include_str!("../assets/house/autumn_house.gltf.part01"),
            include_str!("../assets/house/autumn_house.gltf.part02"),
            include_str!("../assets/house/autumn_house.gltf.part03"),
            include_str!("../assets/house/autumn_house.gltf.part04"),
        ];
        let mut source = String::with_capacity(EXPECTED_BYTES);
        for chunk in chunks {
            source.push_str(chunk);
        }
        assert_eq!(source.len(), EXPECTED_BYTES, "Autumn House asset chunks are incomplete");
        std::fs::write(&target, source).expect("cannot materialize Autumn House asset");
    }
    root.to_string_lossy().into_owned()
}

fn wall(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    let id = s.box_mesh(size, at, 0xffffff, "");
    s.last()["visible"] = json!(false);
    s.obstacles.push(id);
}

/// Conservative collision around the imported shell. Furniture collision and
/// click targets belong to `furnishings`, not to the architectural asset layer.
pub(crate) fn append(s: &mut SceneSpec) {
    wall(s, [0.16, 2.8, 8.10], [-6.22, 1.35, -0.40]);
    wall(s, [0.16, 2.8, 8.10], [4.20, 1.35, -0.40]);
    wall(s, [10.50, 2.8, 0.16], [-1.01, 1.35, -4.48]);
    // Camera-side wall is split so the front opening remains traversable.
    wall(s, [3.80, 2.8, 0.16], [-4.25, 1.35, 3.68]);
    wall(s, [3.80, 2.8, 0.16], [2.25, 1.35, 3.68]);
}
