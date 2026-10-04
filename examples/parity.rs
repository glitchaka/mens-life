//! Machine-readable entry point used only to compare the Rust port with the
//! unmodified original game. It executes the real library, not a test replica.
use mens_life::{
    game::{Bounds, Game, Navigation, Task, Vec3},
    scene::{self, Avatar},
    ui,
};
use serde_json::{Value, json};
use std::io::{self, Read};

fn snapshot(g: &Game) -> Value {
    json!({"values":g.values,"path":g.path,"task":g.task.map(Task::key).unwrap_or(""),"phase":g.phase,"elapsed":g.elapsed,"decisionIn":g.decision_in,"uiTime":g.ui_time,"gameTime":g.game_time,"walkTime":g.walk_time,"action":g.action,"position":g.frame.position,"rotation":g.frame.rotation,"legs":g.frame.legs,"arms":g.frame.arms,"plumb":g.frame.plumb,"drops":g.frame.drops,"dropsY":g.drops_y,"watching":g.frame.watching,"screenHue":g.frame.screen_hue,"displayedValues":g.displayed_values,"displayedClock":g.displayed_clock,"needsVisible":g.needs_visible})
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input: Value = serde_json::from_str(&input).unwrap();
    let avatar: Avatar = input
        .get("avatar")
        .map(|v| serde_json::from_value(v.clone()).unwrap())
        .unwrap_or_default();
    let random: Vec<f64> = input
        .get("random")
        .map(|v| serde_json::from_value(v.clone()).unwrap())
        .unwrap_or_else(|| vec![0.5; 90]);
    let spec = scene::build(&avatar, &random);
    if input["mode"] == "scene" {
        println!("{}", json!({"spec":spec,"ui":ui::shell(&avatar)}));
        return;
    }
    let bounds: Vec<Bounds> = serde_json::from_value(input["bounds"].clone()).unwrap();
    let mut game = Game::new(Navigation::new(bounds), spec.drops_y);
    let mut snapshots = vec![];
    for op in input["ops"].as_array().unwrap() {
        match op["kind"].as_str().unwrap() {
            "command" => game.command(Task::parse(op["task"].as_str().unwrap()).unwrap()),
            "walk" => game.walk_to(serde_json::from_value::<Vec3>(op["point"].clone()).unwrap()),
            "paused" => game.paused = op["value"].as_bool().unwrap(),
            "creator" => game.creator = op["value"].as_bool().unwrap(),
            "values" => game.values = serde_json::from_value(op["value"].clone()).unwrap(),
            "tick" => game.tick(op["dt"].as_f64().unwrap(), op["now"].as_f64().unwrap()),
            _ => panic!("Unknown parity operation"),
        }
        if op["snapshot"].as_bool().unwrap_or(false) {
            snapshots.push(snapshot(&game));
        }
    }
    println!(
        "{}",
        json!({"free":game.navigation.free,"snapshots":snapshots})
    );
}
