//! Navigation collision map for the imported PSX Backroom.
//! Visible architecture comes entirely from `assets/house/house.glb`.
//! The internal collision segments below were extracted from the vertical faces
//! of `Backroom_lambert3_0` after applying the same GLB/root transforms as render.
use crate::scene::SceneSpec;
use serde_json::json;

fn boundary(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3], id: &str) {
    s.box_mesh(size, at, 0xffffff, "");
    s.last()["id"] = json!(id);
    s.last()["visible"] = json!(false);
    s.obstacles.push(id.to_owned());
}

fn wall_x(s: &mut SceneSpec, index: usize, x: f64, z0: f64, z1: f64) {
    let lo = z0.min(z1);
    let hi = z0.max(z1);
    boundary(
        s,
        [0.04, 3.20, hi - lo],
        [x, 1.60, (lo + hi) * 0.5],
        &format!("backroom-wall-x-{index}"),
    );
}

fn wall_z(s: &mut SceneSpec, index: usize, z: f64, x0: f64, x1: f64) {
    let lo = x0.min(x1);
    let hi = x0.max(x1);
    boundary(
        s,
        [hi - lo, 3.20, 0.04],
        [(lo + hi) * 0.5, 1.60, z],
        &format!("backroom-wall-z-{index}"),
    );
}

pub(crate) fn append(s: &mut SceneSpec) {
    // Full centred footprint of the imported GLB.
    let half_x = 23.44871;
    let half_z = 27.50685;
    let thickness = 0.40;
    let wall_height = 4.95;
    let y = wall_height * 0.5;

    boundary(s, [thickness, wall_height, half_z * 2.0 + thickness], [-half_x - thickness * 0.5, y, 0.0], "house-boundary-west");
    boundary(s, [thickness, wall_height, half_z * 2.0 + thickness], [ half_x + thickness * 0.5, y, 0.0], "house-boundary-east");
    boundary(s, [half_x * 2.0 + thickness, wall_height, thickness], [0.0, y, -half_z - thickness * 0.5], "house-boundary-south");
    boundary(s, [half_x * 2.0 + thickness, wall_height, thickness], [0.0, y,  half_z + thickness * 0.5], "house-boundary-north");

    // Vertical wall faces intersecting the active navigation grid. Navigation
    // already adds a 0.30 m clearance around obstacles, so these proxies remain
    // thin and follow the actual mesh instead of inventing room rectangles.
    let x_walls = [
        (-8.051337,-1.008083,0.570900),
        (-7.125233,-8.000000,-0.956890),
        (-6.986785,-8.000000,-0.956890),
        (-6.331280,-1.008083,0.570900),
        (-5.818158,5.966902,7.000000),
        (-5.690144,5.966902,7.000000),
        (-4.840894,3.912725,5.491708),
        (-3.145406,3.912725,5.491708),
        (0.333742,-4.081545,-2.502562),
        (2.049256,-4.081545,-2.502562),
        (2.970915,3.912725,5.491708),
        (4.686430,3.912725,5.491708),
        (9.085879,-4.081545,-2.502562),
        (10.792397,-4.081545,-2.502562),
        (11.714057,3.912725,5.491708),
        (12.640556,-0.827156,7.000000),
        (12.779004,-0.827156,7.000000),
        (13.429572,3.912725,5.491708),
    ];
    for (index, (x, z0, z1)) in x_walls.into_iter().enumerate() {
        wall_x(s, index, x, z0, z1);
    }

    let z_walls = [
        (-7.928004,-7.106571,-2.247736),
        (-7.928004,-0.415782,4.443053),
        (-7.928004,6.275006,11.133842),
        (-7.928004,12.965796,17.000000),
        (-7.789553,-7.106571,-2.247736),
        (-7.789553,-0.415782,4.443053),
        (-7.789553,6.275006,11.133842),
        (-7.789553,12.965796,17.000000),
        (-4.081545,0.333742,2.049256),
        (-4.081545,9.076882,10.792397),
        (-2.502562,0.333742,2.049256),
        (-2.502562,9.076882,10.792397),
        (-1.008083,-8.046795,-6.331280),
        (-0.856099,16.293826,17.000000),
        (-0.717651,16.293826,17.000000),
        (0.570900,-8.046795,-6.331280),
        (3.912725,-4.840894,-3.125380),
        (3.912725,2.970915,4.686430),
        (3.912725,11.714057,13.429572),
        (3.912725,15.109966,17.000000),
        (4.027568,15.109966,17.000000),
        (5.491708,-4.840894,-3.125380),
        (5.491708,2.970915,4.686430),
        (5.491708,11.714057,13.429572),
        (7.464233,16.160518,17.000000),
    ];
    for (index, (z, x0, x1)) in z_walls.into_iter().enumerate() {
        wall_z(s, index, z, x0, x1);
    }
}
