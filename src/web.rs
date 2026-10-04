use crate::{
    game::{Bounds, Game, Navigation, Task, Vec3},
    scene::{self, Avatar},
    ui,
};
use serde::Deserialize;
use serde_json::json;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Document, Element, Event, HtmlInputElement, HtmlSelectElement, PointerEvent};

#[wasm_bindgen(raw_module = "../bridge.js")]
unsafe extern "C" {
    #[wasm_bindgen(catch)]
    fn create_renderer(host: &Element, json: &str) -> Result<String, JsValue>;
    fn apply_updates(json: &str);
    fn render_frame();
    fn pick_scene(x: f64, y: f64) -> String;
}

struct App {
    document: Document,
    avatar: Avatar,
    game: Game,
    last: f64,
    events: Vec<Closure<dyn FnMut(Event)>>,
    needs_count: usize,
    last_action: String,
    last_clock: String,
}
impl App {
    fn element(&self, id: &str) -> Element {
        self.document
            .get_element_by_id(id)
            .expect("Rust UI element")
    }
    fn rebuild(&mut self) -> Result<(), JsValue> {
        let random = (0..90).map(|_| js_sys::Math::random()).collect::<Vec<_>>();
        let spec = scene::build(&self.avatar, &random);
        let bounds = create_renderer(
            &self.element("scene"),
            &serde_json::to_string(&spec).unwrap(),
        )?;
        let bounds: Vec<Bounds> =
            serde_json::from_str(&bounds).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let navigation = Navigation::new(
            bounds
                .into_iter()
                .filter(|b| b.min.y < 1.6 && b.max.y > 0.3)
                .collect(),
        );
        let paused = self.game.paused;
        let creator = self.game.creator;
        let displayed_values = self.game.displayed_values;
        let displayed_clock = self.game.displayed_clock.clone();
        let visible = self.game.needs_visible;
        let action = self.game.action.clone();
        // Avatar edits reset the scene/simulation just like the original useEffect,
        // while the React-era HUD state remains until the next UI update.
        self.game = Game::new(navigation, spec.drops_y);
        self.game.paused = paused;
        self.game.creator = creator;
        self.game.displayed_values = displayed_values;
        self.game.displayed_clock = displayed_clock;
        self.game.needs_visible = visible;
        self.game.action = action;
        self.element("house-name").set_text_content(Some(&format!(
            "CASA DE {}",
            self.avatar.name.to_uppercase()
        )));
        self.element("avatar-name")
            .set_text_content(Some(&self.avatar.name));
        self.element("avatar-letter").set_text_content(Some(
            &self
                .avatar
                .name
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_default(),
        ));
        Ok(())
    }
    fn sync_ui(&mut self) {
        if self.last_action != self.game.action {
            self.element("action")
                .set_text_content(Some(&self.game.action));
            self.last_action = self.game.action.clone();
        }
        if self.last_clock != self.game.displayed_clock {
            self.element("clock")
                .set_text_content(Some(&self.game.displayed_clock));
            self.last_clock = self.game.displayed_clock.clone();
        }
        if self.needs_count != self.game.needs_visible {
            self.element("needs-list").set_inner_html(&ui::needs(
                &self.game.displayed_values,
                self.game.needs_visible,
            ));
            self.needs_count = self.game.needs_visible;
        }
        for i in 0..self.game.needs_visible {
            let bar = self.element(&format!("need-{i}"));
            bar.dyn_ref::<web_sys::HtmlElement>()
                .unwrap()
                .style()
                .set_property("width", &format!("{}%", self.game.displayed_values[i]))
                .unwrap();
        }
    }
    fn render(&mut self, now: f64) {
        let dt = ((now - self.last) / 1000.0).min(0.05);
        self.last = now;
        self.game.tick(dt, now);
        let f = &self.game.frame;
        let mut updates = vec![
            json!({"id":"person","position":f.position.array(),"rotation":f.rotation}),
            json!({"id":"leg-0","rotation":[f.legs[0],0,0]}),
            json!({"id":"leg-1","rotation":[f.legs[1],0,0]}),
            json!({"id":"arm-0","rotation":[f.arms[0],0,0]}),
            json!({"id":"arm-1","rotation":[f.arms[1],0,0]}),
            json!({"id":"plumb","visible":f.plumb}),
            json!({"id":"drops","visible":f.drops}),
            json!({"id":"tv-screen","material":{"color":if f.watching {0x79a7d3}else{0x17212b},"emissive_hsl":[f.screen_hue,0.5,if f.watching {0.4}else{0.0}]}}),
        ];
        if f.drops {
            for (i, y) in self.game.drops_y.iter().enumerate() {
                updates.push(json!({"id":format!("drop-{i}"),"y":y}));
            }
        }
        apply_updates(&serde_json::to_string(&updates).unwrap());
        self.sync_ui();
        render_frame();
    }
}
#[derive(Deserialize)]
struct Pick {
    names: Vec<String>,
    floor: Option<Vec3>,
}

fn listen(
    app: &Rc<RefCell<App>>,
    id: &str,
    event: &str,
    callback: impl Fn(&mut App, Event) + 'static,
) -> Result<(), JsValue> {
    let target = app.borrow().element(id);
    let state = app.clone();
    let closure = Closure::wrap(Box::new(move |e: Event| {
        callback(&mut state.borrow_mut(), e);
    }) as Box<dyn FnMut(Event)>);
    target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())?;
    app.borrow_mut().events.push(closure);
    Ok(())
}

type Animation = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Window unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Document unavailable"))?;
    let avatar = Avatar::default();
    document
        .get_element_by_id("app")
        .unwrap()
        .set_inner_html(&ui::shell(&avatar));
    let game = Game::new(Navigation::new(vec![]), vec![]);
    let app = Rc::new(RefCell::new(App {
        document,
        avatar,
        game,
        last: window.performance().unwrap().now(),
        events: vec![],
        needs_count: 4,
        last_action: "Nada pendiente".into(),
        last_clock: "09:42".into(),
    }));
    app.borrow_mut().rebuild()?;
    listen(&app, "pause", "click", |app, _| {
        app.game.paused = !app.game.paused;
        app.element("pause")
            .set_text_content(Some(if app.game.paused { "▶" } else { "Ⅱ" }));
    })?;
    listen(&app, "play", "click", |app, _| {
        app.game.creator = false;
        app.element("creator").set_attribute("hidden", "").unwrap();
    })?;
    listen(&app, "menu", "click", |app, _| {
        app.game.creator = true;
        app.element("creator").remove_attribute("hidden").unwrap();
    })?;
    for task in [Task::Eat, Task::Tv, Task::Bed, Task::Shower, Task::Toilet] {
        listen(&app, task.key(), "click", move |app, _| {
            app.game.command(task);
            app.sync_ui();
        })?;
    }
    listen(&app, "scene", "pointerdown", |app, e| {
        if app.game.creator || app.game.paused {
            return;
        }
        let Some(e) = e.dyn_ref::<PointerEvent>() else {
            return;
        };
        let Ok(hit) =
            serde_json::from_str::<Pick>(&pick_scene(e.client_x() as f64, e.client_y() as f64))
        else {
            return;
        };
        if let Some(task) = hit.names.iter().find_map(|name| Task::parse(name)) {
            app.game.command(task);
        } else if let Some(point) = hit.floor {
            app.game.walk_to(point);
        }
        app.sync_ui();
    })?;
    for id in ["name", "body", "skin", "hair", "shirt", "pants"] {
        listen(
            &app,
            id,
            if id == "body" { "change" } else { "input" },
            move |app, e| {
                let target = e.target().unwrap();
                let value = if id == "body" {
                    target.dyn_ref::<HtmlSelectElement>().unwrap().value()
                } else {
                    target.dyn_ref::<HtmlInputElement>().unwrap().value()
                };
                match id {
                    "name" => {
                        app.avatar.name = if value.is_empty() {
                            "Alex".into()
                        } else {
                            value
                        };
                        app.element("name")
                            .dyn_ref::<HtmlInputElement>()
                            .unwrap()
                            .set_value(&app.avatar.name);
                    }
                    "body" => app.avatar.body = value,
                    "skin" => app.avatar.skin = value,
                    "hair" => app.avatar.hair = value,
                    "shirt" => app.avatar.shirt = value,
                    "pants" => app.avatar.pants = value,
                    _ => unreachable!(),
                }
                if let Err(error) = app.rebuild() {
                    web_sys::console::error_1(&error);
                }
            },
        )?;
    }
    let animation: Animation = Rc::new(RefCell::new(None));
    let next = animation.clone();
    *animation.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f64| {
        app.borrow_mut().render(now);
        web_sys::window()
            .unwrap()
            .request_animation_frame(next.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut(f64)>));
    window.request_animation_frame(
        animation
            .borrow()
            .as_ref()
            .unwrap()
            .as_ref()
            .unchecked_ref(),
    )?;
    // The retained closure owns the app for the lifetime of this document.
    Ok(())
}
