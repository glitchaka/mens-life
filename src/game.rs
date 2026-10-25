use serde::{Deserialize, Serialize};
use crate::psychology::{ActivityEvent, Expression, Psychology, Reaction};
use std::collections::VecDeque;
use std::f64::consts::PI;

pub const MIN_X: f64 = -9.0;
pub const MAX_X: f64 = 17.0;
pub const MIN_Z: f64 = -8.0;
pub const MAX_Z: f64 = 7.0;
pub const WIDTH: usize = 131;
pub const DEPTH: usize = 76;
pub const STEP: f64 = 0.2;
pub const INITIAL: [f64; 5] = [74.0, 61.0, 82.0, 55.0, 70.0];
const DECAY: [f64; 5] = [0.16, 0.23, 0.13, 0.2, 0.18];

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    pub fn array(self) -> [f64; 3] { [self.x, self.y, self.z] }
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        Self::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Bounds { pub min: Vec3, pub max: Vec3 }

#[derive(Clone, Debug)]
pub struct Navigation { pub bounds: Vec<Bounds>, pub free: Vec<bool> }
impl Navigation {
    pub fn new(bounds: Vec<Bounds>) -> Self {
        let mut result = Self { bounds, free: Vec::new() };
        result.free = (0..WIDTH * DEPTH).map(|i| {
            let p = Self::point(i);
            !result.blocked(p.x, p.z)
        }).collect();
        result
    }
    pub fn point(i: usize) -> Vec3 {
        Vec3::new(MIN_X + (i % WIDTH) as f64 * STEP, 0.0, MIN_Z + (i / WIDTH) as f64 * STEP)
    }
    pub fn blocked(&self, x: f64, z: f64) -> bool {
        !(MIN_X + 0.05..=MAX_X - 0.05).contains(&x)
            || !(MIN_Z + 0.05..=MAX_Z - 0.05).contains(&z)
            || self.bounds.iter().any(|b| {
                x > b.min.x - 0.3 && x < b.max.x + 0.3 && z > b.min.z - 0.3 && z < b.max.z + 0.3
            })
    }
    pub fn nearest(&self, p: Vec3) -> Option<usize> {
        let mut best = None;
        let mut distance = f64::INFINITY;
        for (i, free) in self.free.iter().enumerate() {
            if !free { continue; }
            let v = Self::point(i);
            let d = (p.x - v.x).powi(2) + (p.z - v.z).powi(2);
            if d < distance { best = Some(i); distance = d; }
        }
        best
    }
    pub fn route(&self, from: Vec3, to: Vec3) -> Option<VecDeque<Vec3>> {
        let start = self.nearest(from)?;
        let end = self.nearest(to)?;
        if self.blocked(to.x, to.z) || !self.segment_free(from, Self::point(start)) { return None; }
        let mut prev = vec![None; self.free.len()];
        let mut queue = VecDeque::from([start]);
        prev[start] = Some(start);
        while prev[end].is_none() {
            let i = queue.pop_front()?;
            for n in [i.checked_sub(1), i.checked_add(1), i.checked_sub(WIDTH), i.checked_add(WIDTH)]
                .into_iter().flatten()
            {
                if n >= self.free.len()
                    || !self.free[n]
                    || prev[n].is_some()
                    || (n % WIDTH).abs_diff(i % WIDTH) > 1
                    || !self.segment_free(Self::point(i), Self::point(n))
                { continue; }
                prev[n] = Some(i);
                queue.push_back(n);
            }
        }
        let mut result = VecDeque::new();
        let mut i = end;
        while i != start { result.push_front(Self::point(i)); i = prev[i]?; }
        if (from.x - Self::point(start).x).hypot(from.z - Self::point(start).z) > 0.001 {
            result.push_front(Self::point(start));
        }
        Some(result)
    }
    pub fn segment_free(&self, from: Vec3, to: Vec3) -> bool {
        let distance = (from.x - to.x).hypot(from.z - to.z);
        let steps = (distance / 0.04).ceil().max(1.0) as usize;
        (0..=steps).all(|i| {
            let p = Vec3::lerp(from, to, i as f64 / steps as f64);
            !self.blocked(p.x, p.z)
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Task { Bed, Tv, Shower, Toilet, Eat }
impl Task {
    pub fn parse(s: &str) -> Option<Self> {
        match s { "bed" => Some(Self::Bed), "tv" => Some(Self::Tv), "shower" => Some(Self::Shower), "toilet" => Some(Self::Toilet), "eat" => Some(Self::Eat), _ => None }
    }
    pub fn key(self) -> &'static str {
        match self { Self::Bed => "bed", Self::Tv => "tv", Self::Shower => "shower", Self::Toilet => "toilet", Self::Eat => "eat" }
    }
    pub fn spec(self) -> Spec {
        match self {
            Self::Bed => Spec { approach: Vec3::new(-2.8, 0.0, 2.2), pose: Vec3::new(-4.6, 1.20, 2.1), need: 0, label: "Durmiendo", going: "Yendo a la cama", duration: 18.0 },
            Self::Tv => Spec { approach: Vec3::new(2.2, 0.0, 1.9), pose: Vec3::new(2.6, -0.36, 3.12), need: 3, label: "Viendo televisión", going: "Yendo a ver televisión", duration: 16.0 },
            Self::Shower => Spec { approach: Vec3::new(-5.9, 0.0, -2.8), pose: Vec3::new(-5.9, 0.22, -4.1), need: 2, label: "Duchándose", going: "Yendo a la ducha", duration: 12.0 },
            Self::Toilet => Spec { approach: Vec3::new(-3.8, 0.0, -3.3), pose: Vec3::new(-3.65, -0.32, -4.2), need: 4, label: "Usando el baño", going: "Yendo al baño", duration: 9.0 },
            Self::Eat => Spec { approach: Vec3::new(3.05, 0.0, -1.05), pose: Vec3::new(3.05, 0.0, -1.05), need: 1, label: "Comiendo", going: "Yendo a comer", duration: 10.0 },
        }
    }
}
pub struct Spec { pub approach: Vec3, pub pose: Vec3, pub need: usize, pub label: &'static str, pub going: &'static str, pub duration: f64 }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase { Idle, Walking, Using, Exiting }

#[derive(Clone, Debug, Serialize)]
pub struct Frame {
    pub position: Vec3,
    pub rotation: [f64; 3],
    pub legs: [f64; 2],
    pub arms: [f64; 2],
    pub knees: [f64; 2],
    pub elbows: [f64; 2],
    pub spine: [f64; 3],
    pub head: [f64; 2],
    pub breath: f64,
    pub plumb: bool,
    pub drops: bool,
    pub watching: bool,
    pub screen_hue: f64,
}
impl Default for Frame {
    fn default() -> Self {
        Self { position: Vec3::new(0.0, 0.0, 2.0), rotation: [0.0; 3], legs: [0.0; 2], arms: [0.0; 2], knees: [0.04; 2], elbows: [0.12; 2], spine: [0.0; 3], head: [0.0; 2], breath: 0.0, plumb: true, drops: false, watching: false, screen_hue: 0.0 }
    }
}
impl Frame {
    pub fn gait(&mut self, time: f64, moving: bool) {
        self.breath = (time * 1.8).sin() * 0.003;
        if moving {
            let stride = time * 7.2;
            for i in 0..2 {
                let wave = (stride + i as f64 * PI).sin();
                self.legs[i] = wave * 0.38;
                self.knees[i] = wave.max(0.0) * 0.65 + 0.05;
                self.arms[i] = -wave * 0.23;
                self.elbows[i] = 0.16 + wave.abs() * 0.1;
            }
            self.spine = [0.02, stride.sin() * 0.035, (stride * 0.5).sin() * 0.015];
        } else {
            self.legs = [0.0; 2];
            self.knees = [0.04; 2];
            self.arms = [-0.03; 2];
            self.elbows = [0.12; 2];
            self.spine = [0.0; 3];
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub navigation: Navigation,
    pub values: [f64; 5],
    pub path: VecDeque<Vec3>,
    pub task: Option<Task>,
    pub phase: Phase,
    pub elapsed: f64,
    pub decision_in: f64,
    pub ui_time: f64,
    pub game_time: f64,
    pub walk_time: f64,
    pub action: String,
    pub paused: bool,
    pub creator: bool,
    pub needs_visible: usize,
    pub displayed_values: [f64; 5],
    pub displayed_clock: String,
    pub frame: Frame,
    pub drops_y: Vec<f64>,
    pub psychology: Psychology,
    pub reaction: Reaction,
    pub ground_position: Vec3,
    pub talking: bool,
    pub order_serial: u64,
    manual_order: bool,
    reconsider_in: f64,
    activity_start: f64,
    blocked_until: [f64; 5],
    simulation_time: f64,
    walking_for: f64,
    entry_position: Vec3,
    exit_pose: Frame,
}
impl Game {
    pub fn new(navigation: Navigation, drops_y: Vec<f64>) -> Self {
        let position = navigation.nearest(Frame::default().position).map(Navigation::point).unwrap_or(Frame::default().position);
        Self {
            navigation, values: INITIAL, path: VecDeque::new(), task: None, phase: Phase::Idle, elapsed: 0.0, decision_in: 1.0,
            ui_time: 0.0, game_time: 582.0, walk_time: 0.0, action: "Nada pendiente".into(), paused: false, creator: true,
            needs_visible: 4, displayed_values: INITIAL, displayed_clock: "09:42".into(), frame: Frame { position, ..Frame::default() },
            drops_y, psychology: Psychology::new(0x4d454e535f4c4946).expect("No se pudo iniciar MSPA-Lib para el personaje"),
            reaction: Reaction::default(), ground_position: position, talking: false, order_serial: 0, manual_order: false,
            reconsider_in: 2.0, activity_start: 0.0, blocked_until: [0.0; 5], simulation_time: 0.0, walking_for: 0.0,
            entry_position: position, exit_pose: Frame::default(),
        }
    }
    pub fn restore(&mut self) {
        self.frame.position = self.ground_position;
        self.frame.rotation = [0.0; 3];
        self.frame.position.y = 0.0;
        self.frame.legs = [0.0; 2];
        self.frame.arms = [0.0; 2];
        self.frame.knees = [0.04; 2];
        self.frame.elbows = [0.12; 2];
        self.frame.spine = [0.0; 3];
        self.frame.plumb = true;
    }
    pub fn command(&mut self, task: Task) { self.begin_task(task, true); }
    fn interrupt(&mut self) {
        if self.phase != Phase::Exiting && let Some(task) = self.task { self.psychology.event(task, ActivityEvent::Interrupted); }
    }
    fn begin_task(&mut self, task: Task, manual: bool) {
        if manual { self.order_serial = self.order_serial.wrapping_add(1); }
        self.talking = false;
        self.interrupt();
        self.restore();
        let Some(next) = self.navigation.route(self.ground_position, task.spec().approach) else {
            self.path.clear(); self.phase = Phase::Idle; self.task = None; self.action = "No hay un camino libre".into(); self.decision_in = 2.0;
            self.blocked_until[task.spec().need] = self.simulation_time + 10.0;
            self.psychology.event(task, ActivityEvent::Failed);
            self.say("No puedo llegar hasta ahí. Buscaré otra cosa.");
            return;
        };
        self.task = Some(task); self.path = next; self.phase = Phase::Walking; self.elapsed = 0.0; self.walking_for = 0.0;
        self.action = task.spec().going.into(); self.manual_order = manual; self.reconsider_in = 2.0; self.activity_start = self.values[task.spec().need];
        self.psychology.event(task, ActivityEvent::Started);
        self.say(match task { Task::Bed => "Voy a descansar un rato.", Task::Eat => "¡Hora de comer algo!", Task::Shower => "Me voy a dar una buena ducha.", Task::Tv => "A ver qué ponen en la tele.", Task::Toilet => "Voy al baño. Ahora vuelvo." });
    }
    pub fn walk_to(&mut self, point: Vec3) {
        if self.creator || self.paused || self.navigation.blocked(point.x, point.z) { return; }
        let from = self.ground_position;
        if let Some(next) = self.navigation.route(from, point) {
            self.order_serial = self.order_serial.wrapping_add(1); self.interrupt(); self.restore(); self.talking = false;
            self.path = next; self.task = None; self.phase = Phase::Walking; self.action = "Caminando".into(); self.manual_order = true; self.walking_for = 0.0;
            self.say("Voy para allá.");
        } else { self.say("No encuentro un camino hasta allí."); }
    }
    fn consider(&mut self) {
        let from = self.ground_position;
        let mut available = Vec::new();
        for task in [Task::Bed, Task::Eat, Task::Shower, Task::Tv, Task::Toilet] {
            let spec = task.spec();
            if self.simulation_time < self.blocked_until[spec.need] { continue; }
            if let Some(route) = self.navigation.route(from, spec.approach) {
                let cost = route.len() as f32 * STEP as f32 * 0.003 + spec.duration as f32 * 0.002;
                let progress = if self.task == Some(task) && self.phase == Phase::Using { (self.elapsed / spec.duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
                available.push((task, cost, progress));
            }
        }
        if let Some(task) = self.psychology.decide(&available, self.task) {
            if self.task != Some(task) { self.begin_task(task, false); }
        }
        self.decision_in = 2.0;
    }
    pub fn expression(&self) -> Expression {
        if self.psychology.error.is_some() { return Expression::Anxious; }
        if self.phase == Phase::Using && self.task == Some(Task::Bed) { return Expression::Sleeping; }
        let lowest = (0..5).min_by(|a, b| self.values[*a].total_cmp(&self.values[*b])).unwrap();
        if self.values[lowest] <= 35.0 { return [Expression::Tired, Expression::Hungry, Expression::Dirty, Expression::Bored, Expression::Urgent][lowest]; }
        self.psychology.emotion()
    }
    pub fn say(&mut self, words: &str) {
        self.reaction = Reaction { expression: self.expression(), words: words.into(), remaining: 7.0 };
    }
    fn refresh_reaction(&mut self, dt: f64) {
        self.reaction.remaining -= dt;
        let next = self.expression();
        if self.reaction.remaining <= 0.0 || (next != self.reaction.expression && self.reaction.remaining < 3.0) {
            self.reaction.words = if self.psychology.error.is_some() { "No consigo decidir qué hacer ahora.".into() } else { next.words().into() };
            self.reaction.remaining = 10.0;
        }
        self.reaction.expression = next;
    }
    pub fn tick(&mut self, dt: f64, now: f64) {
        if self.paused || self.creator { return; }
        let dt = if dt.is_finite() { dt.clamp(0.0, 0.05) } else { 0.0 };
        self.simulation_time += dt;
        self.game_time += dt * 1.25;
        self.ui_time += dt;
        for (i, value) in self.values.iter_mut().enumerate() { *value = (*value - dt * DECAY[i]).max(0.0); }
        self.psychology.advance(dt, self.values);
        if self.phase == Phase::Idle && !self.talking {
            self.decision_in -= dt;
            if self.decision_in <= 0.0 { self.consider(); }
        } else if self.phase != Phase::Exiting && !self.manual_order && self.task.is_some() {
            self.reconsider_in -= dt;
            if self.reconsider_in <= 0.0 { self.reconsider_in = 2.0; self.consider(); }
        }
        if self.phase == Phase::Walking {
            self.walking_for += dt;
            if let Some(point) = self.path.front().copied() {
                let dx = point.x - self.ground_position.x;
                let dz = point.z - self.ground_position.z;
                let distance = (dx * dx + dz * dz).sqrt();
                let movement = distance.min(dt * 1.65);
                if distance < 0.02 {
                    self.ground_position = point;
                    self.path.pop_front();
                } else {
                    let x = self.ground_position.x + dx * (movement / distance);
                    let z = self.ground_position.z + dz * (movement / distance);
                    let next = Vec3::new(x, 0.0, z);
                    if self.walking_for <= 45.0 && self.navigation.segment_free(self.ground_position, next) {
                        self.ground_position = next;
                        self.frame.rotation[1] = dx.atan2(dz);
                    } else {
                        if let Some(task) = self.task {
                            self.psychology.event(task, ActivityEvent::Failed);
                            self.blocked_until[task.spec().need] = self.simulation_time + 10.0;
                        }
                        self.path.clear(); self.task = None; self.phase = Phase::Idle; self.decision_in = 2.0;
                        self.action = "El camino está bloqueado".into(); self.say("Vaya, no puedo pasar por aquí.");
                    }
                }
                self.frame.position = self.ground_position;
                self.walk_time += dt;
                self.frame.gait(self.simulation_time, self.phase == Phase::Walking);
            } else {
                self.frame.legs = [0.0; 2];
                self.frame.arms = [0.0; 2];
                if let Some(task) = self.task {
                    self.phase = Phase::Using; self.entry_position = self.ground_position; self.elapsed = 0.0; self.action = task.spec().label.into();
                    self.say(match task { Task::Bed => "Zzz... necesitaba este descanso.", Task::Eat => "Mmm... ¡qué rico!", Task::Shower => "¡Qué gusto el agua!", Task::Tv => "Este programa parece entretenido.", Task::Toilet => "Un momento de intimidad, por favor." });
                } else {
                    self.phase = Phase::Idle; self.decision_in = 2.0; self.action = "Descansando".into(); self.manual_order = false;
                }
            }
        }
        if self.phase == Phase::Using && let Some(task) = self.task {
            let spec = task.spec();
            self.elapsed += dt;
            let t = (self.elapsed / 1.1).min(1.0);
            let ease = t * t * (3.0 - 2.0 * t);
            self.frame.position = Vec3::lerp(self.entry_position, spec.pose, ease);
            self.frame.gait(self.simulation_time, false);
            match task {
                Task::Bed => { self.frame.rotation = [PI / 2.0 * ease, PI * ease, 0.0]; self.frame.knees = [0.08 * ease; 2]; self.frame.arms = [-0.12 * ease; 2]; self.frame.plumb = false; }
                Task::Tv | Task::Toilet => { self.frame.rotation = [0.0, if task == Task::Tv { PI } else { 0.0 }, 0.0]; self.frame.legs = [-PI / 2.0 * t; 2]; self.frame.knees = [PI / 2.0 * t; 2]; self.frame.elbows = [0.65 * t; 2]; self.frame.arms = [-0.3 * t; 2]; }
                Task::Shower => { self.frame.arms = [-1.2 + (self.elapsed * 2.0).sin() * 0.2, -0.9 - (self.elapsed * 2.0).sin() * 0.2]; self.frame.elbows = [1.0, 1.2]; }
                Task::Eat => { self.frame.rotation[1] = PI; self.frame.arms[1] = -0.55 + (self.elapsed * 2.0).sin() * 0.18; self.frame.elbows[1] = 1.7 + (self.elapsed * 2.0).sin() * 0.25; }
            }
            if t == 1.0 { self.values[spec.need] = (self.values[spec.need] + dt * 5.0).min(100.0); }
            if self.elapsed >= spec.duration {
                let improved = self.values[spec.need] > self.activity_start + 0.5 || self.values[spec.need] >= 95.0;
                self.psychology.event(task, if improved { ActivityEvent::Completed } else { ActivityEvent::Failed });
                self.exit_pose = self.frame.clone(); self.phase = Phase::Exiting; self.elapsed = 0.0; self.decision_in = 2.0; self.action = "Descansando".into(); self.manual_order = false;
                self.say(if improved { match task { Task::Bed => "¡Mucho mejor! Ya tengo más energía.", Task::Eat => "¡Qué bien! Ya no tengo hambre.", Task::Shower => "Ahora sí, limpio y como nuevo.", Task::Tv => "Me hacía falta desconectar un poco.", Task::Toilet => "Uf... ¡qué alivio!" } } else { "Eso no salió como esperaba." });
            }
        }
        if self.phase == Phase::Exiting {
            self.elapsed += dt;
            let t = (self.elapsed / 0.65).min(1.0);
            let t = t * t * (3.0 - 2.0 * t);
            self.frame.position = Vec3::lerp(self.exit_pose.position, self.ground_position, t);
            for i in 0..3 { self.frame.rotation[i] = self.exit_pose.rotation[i] * (1.0 - t); }
            for i in 0..2 {
                self.frame.legs[i] = self.exit_pose.legs[i] * (1.0 - t);
                self.frame.knees[i] = self.exit_pose.knees[i] * (1.0 - t);
                self.frame.arms[i] = self.exit_pose.arms[i] * (1.0 - t);
                self.frame.elbows[i] = self.exit_pose.elbows[i] * (1.0 - t) + 0.12 * t;
            }
            if t >= 1.0 { self.restore(); self.task = None; self.phase = Phase::Idle; self.decision_in = 2.0; }
        }
        if self.phase == Phase::Idle { self.frame.gait(self.simulation_time, false); }
        self.frame.drops = self.phase == Phase::Using && self.task == Some(Task::Shower);
        if self.frame.drops {
            for y in &mut self.drops_y { *y -= dt * 3.0; if *y < 0.3 { *y = 2.5; } }
        }
        self.frame.watching = self.phase == Phase::Using && self.task == Some(Task::Tv);
        self.frame.screen_hue = (now / 14000.0) % 1.0;
        if self.ui_time > 0.3 {
            self.ui_time = 0.0; self.needs_visible = 5; self.displayed_values = self.values;
            let m = self.game_time.floor() as u64 % 1440;
            self.displayed_clock = format!("{:02}:{:02}", m / 60, m % 60);
        }
        self.refresh_reaction(dt);
    }
}
