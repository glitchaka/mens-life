use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::f64::consts::PI;

pub const WIDTH: usize = 71;
pub const DEPTH: usize = 51;
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
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        Self::new(
            a.x + (b.x - a.x) * t,
            a.y + (b.y - a.y) * t,
            a.z + (b.z - a.z) * t,
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Clone, Debug)]
pub struct Navigation {
    pub bounds: Vec<Bounds>,
    pub free: Vec<bool>,
}
impl Navigation {
    pub fn new(mut bounds: Vec<Bounds>) -> Self {
        bounds.extend([
            Bounds {
                min: Vec3::new(-6.8, 0.0, 1.075),
                max: Vec3::new(-1.7, 2.0, 4.325),
            },
            Bounds {
                min: Vec3::new(-3.8, 0.0, -5.2),
                max: Vec3::new(-2.7, 2.0, -3.58),
            },
            Bounds {
                min: Vec3::new(-4.74, 0.0, -5.1),
                max: Vec3::new(-4.62, 3.0, -3.3),
            },
        ]);
        let mut result = Self {
            bounds,
            free: Vec::new(),
        };
        result.free = (0..WIDTH * DEPTH)
            .map(|i| {
                let p = Self::point(i);
                !result.blocked(p.x, p.z)
            })
            .collect();
        result
    }
    pub fn point(i: usize) -> Vec3 {
        Vec3::new(
            -7.0 + (i % WIDTH) as f64 * STEP,
            0.0,
            -5.0 + (i / WIDTH) as f64 * STEP,
        )
    }
    pub fn blocked(&self, x: f64, z: f64) -> bool {
        !(-6.95..=6.95).contains(&x)
            || !(-4.95..=4.95).contains(&z)
            || self.bounds.iter().any(|b| {
                x > b.min.x - 0.3 && x < b.max.x + 0.3 && z > b.min.z - 0.3 && z < b.max.z + 0.3
            })
    }
    pub fn nearest(&self, p: Vec3) -> Option<usize> {
        let mut best = None;
        let mut distance = f64::INFINITY;
        for (i, free) in self.free.iter().enumerate() {
            if !free {
                continue;
            }
            let v = Self::point(i);
            let d = (p.x - v.x).powi(2) + (p.z - v.z).powi(2);
            if d < distance {
                best = Some(i);
                distance = d;
            }
        }
        best
    }
    pub fn route(&self, from: Vec3, to: Vec3) -> Option<VecDeque<Vec3>> {
        let start = self.nearest(from)?;
        let end = self.nearest(to)?;
        let mut prev = vec![None; self.free.len()];
        let mut queue = VecDeque::from([start]);
        prev[start] = Some(start);
        while prev[end].is_none() {
            let i = queue.pop_front()?;
            for n in [
                i.checked_sub(1),
                i.checked_add(1),
                i.checked_sub(WIDTH),
                i.checked_add(WIDTH),
            ]
            .into_iter()
            .flatten()
            {
                if n >= self.free.len()
                    || !self.free[n]
                    || prev[n].is_some()
                    || (n % WIDTH).abs_diff(i % WIDTH) > 1
                {
                    continue;
                }
                prev[n] = Some(i);
                queue.push_back(n);
            }
        }
        let mut result = VecDeque::new();
        let mut i = end;
        while i != start {
            result.push_front(Self::point(i));
            i = prev[i]?;
        }
        Some(result)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Task {
    Bed,
    Tv,
    Shower,
    Toilet,
    Eat,
}
impl Task {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "bed" => Some(Self::Bed),
            "tv" => Some(Self::Tv),
            "shower" => Some(Self::Shower),
            "toilet" => Some(Self::Toilet),
            "eat" => Some(Self::Eat),
            _ => None,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            Self::Bed => "bed",
            Self::Tv => "tv",
            Self::Shower => "shower",
            Self::Toilet => "toilet",
            Self::Eat => "eat",
        }
    }
    pub fn spec(self) -> Spec {
        match self {
            Self::Bed => Spec {
                approach: Vec3::new(-4.3, 0.0, 0.4),
                pose: Vec3::new(-4.3, 1.5, 1.25),
                need: 0,
                label: "Durmiendo",
                going: "Yendo a la cama",
                duration: 18.0,
            },
            Self::Tv => Spec {
                approach: Vec3::new(0.65, 0.0, 3.2),
                pose: Vec3::new(1.9, 0.55, 3.12),
                need: 3,
                label: "Viendo televisión",
                going: "Yendo a ver televisión",
                duration: 16.0,
            },
            Self::Shower => Spec {
                approach: Vec3::new(-5.5, 0.0, -3.0),
                pose: Vec3::new(-5.7, 0.23, -4.1),
                need: 2,
                label: "Duchándose",
                going: "Yendo a la ducha",
                duration: 12.0,
            },
            Self::Toilet => Spec {
                approach: Vec3::new(-4.25, 0.0, -3.1),
                pose: Vec3::new(-3.25, 0.25, -4.05),
                need: 4,
                label: "Usando el baño",
                going: "Yendo a el baño",
                duration: 9.0,
            },
            Self::Eat => Spec {
                approach: Vec3::new(1.3, 0.0, -2.0),
                pose: Vec3::new(1.3, 0.0, -2.0),
                need: 1,
                label: "Comiendo",
                going: "Yendo a comer",
                duration: 10.0,
            },
        }
    }
}
pub struct Spec {
    pub approach: Vec3,
    pub pose: Vec3,
    pub need: usize,
    pub label: &'static str,
    pub going: &'static str,
    pub duration: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Idle,
    Walking,
    Using,
}

#[derive(Clone, Debug, Serialize)]
pub struct Frame {
    pub position: Vec3,
    pub rotation: [f64; 3],
    pub legs: [f64; 2],
    pub arms: [f64; 2],
    pub plumb: bool,
    pub drops: bool,
    pub watching: bool,
    pub screen_hue: f64,
}
impl Default for Frame {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 2.0),
            rotation: [0.0; 3],
            legs: [0.0; 2],
            arms: [0.0; 2],
            plumb: true,
            drops: false,
            watching: false,
            screen_hue: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
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
}
impl Game {
    pub fn new(navigation: Navigation, drops_y: Vec<f64>) -> Self {
        Self {
            navigation,
            values: INITIAL,
            path: VecDeque::new(),
            task: None,
            phase: Phase::Idle,
            elapsed: 0.0,
            decision_in: 1.0,
            ui_time: 0.0,
            game_time: 582.0,
            walk_time: 0.0,
            action: "Nada pendiente".into(),
            paused: false,
            creator: true,
            needs_visible: 4,
            displayed_values: INITIAL,
            displayed_clock: "09:42".into(),
            frame: Frame::default(),
            drops_y,
        }
    }
    pub fn restore(&mut self) {
        if self.phase == Phase::Using
            && let Some(task) = self.task
            && let Some(i) = self.navigation.nearest(task.spec().approach)
        {
            self.frame.position = Navigation::point(i);
        }
        self.frame.rotation = [0.0; 3];
        self.frame.position.y = 0.0;
        self.frame.legs = [0.0; 2];
        self.frame.arms = [0.0; 2];
        self.frame.plumb = true;
    }
    pub fn command(&mut self, task: Task) {
        self.restore();
        let Some(next) = self
            .navigation
            .route(self.frame.position, task.spec().approach)
        else {
            self.phase = Phase::Idle;
            self.task = None;
            self.action = "No hay un camino libre".into();
            return;
        };
        self.task = Some(task);
        self.path = next;
        self.phase = Phase::Walking;
        self.elapsed = 0.0;
        self.action = task.spec().going.into();
    }
    pub fn walk_to(&mut self, point: Vec3) {
        if self.creator || self.paused || self.navigation.blocked(point.x, point.z) {
            return;
        }
        self.restore();
        if let Some(next) = self.navigation.route(self.frame.position, point) {
            self.path = next;
            self.task = None;
            self.phase = Phase::Walking;
            self.action = "Caminando".into();
        }
    }
    pub fn tick(&mut self, dt: f64, now: f64) {
        if self.paused || self.creator {
            return;
        }
        let dt = dt.min(0.05);
        self.game_time += dt * 1.25;
        self.ui_time += dt;
        for (i, value) in self.values.iter_mut().enumerate() {
            *value = (*value - dt * DECAY[i]).max(0.0);
        }
        if self.phase == Phase::Idle {
            self.decision_in -= dt;
            if self.decision_in <= 0.0 {
                let mut lowest = 0;
                for i in 1..5 {
                    if self.values[i] < self.values[lowest] {
                        lowest = i;
                    }
                }
                self.command([Task::Bed, Task::Eat, Task::Shower, Task::Tv, Task::Toilet][lowest]);
            }
        }
        if self.phase == Phase::Walking {
            if let Some(point) = self.path.front().copied() {
                let dx = point.x - self.frame.position.x;
                let dz = point.z - self.frame.position.z;
                let distance = (dx * dx + dz * dz).sqrt();
                let movement = distance.min(dt * 1.65);
                if distance < 0.02 {
                    self.path.pop_front();
                } else {
                    let x = self.frame.position.x + dx * (movement / distance);
                    let z = self.frame.position.z + dz * (movement / distance);
                    if !self.navigation.blocked(x, z) {
                        self.frame.position.x = x;
                        self.frame.position.z = z;
                        self.frame.rotation[1] = dx.atan2(dz);
                    } else {
                        self.path.clear();
                        self.task = None;
                        self.phase = Phase::Idle;
                        self.decision_in = 2.0;
                    }
                }
                self.walk_time += dt * 8.0;
                self.frame.legs = [self.walk_time.sin() * 0.45, -self.walk_time.sin() * 0.45];
                self.frame.arms = [-self.walk_time.sin() * 0.35, self.walk_time.sin() * 0.35];
            } else {
                self.frame.legs = [0.0; 2];
                self.frame.arms = [0.0; 2];
                if let Some(task) = self.task {
                    self.phase = Phase::Using;
                    self.elapsed = 0.0;
                    self.action = task.spec().label.into();
                } else {
                    self.phase = Phase::Idle;
                    self.decision_in = 2.0;
                    self.action = "Descansando".into();
                }
            }
        }
        if self.phase == Phase::Using
            && let Some(task) = self.task
        {
            let spec = task.spec();
            self.elapsed += dt;
            let t = (self.elapsed / 1.1).min(1.0);
            self.frame.position = Vec3::lerp(spec.approach, spec.pose, t);
            match task {
                Task::Bed => {
                    self.frame.rotation = [PI / 2.0 * t, 0.0, 0.0];
                    self.frame.plumb = false;
                }
                Task::Tv | Task::Toilet => {
                    self.frame.rotation = [0.0, if task == Task::Tv { PI } else { 0.0 }, 0.0];
                    self.frame.legs = [-PI / 2.0 * t; 2];
                    self.frame.arms = [-0.5 * t; 2];
                }
                Task::Shower => {
                    self.frame.arms = [
                        -1.7 + (self.elapsed * 3.0).sin() * 0.3,
                        -1.2 - (self.elapsed * 3.0).sin() * 0.3,
                    ];
                }
                Task::Eat => {
                    self.frame.arms[1] = -1.4 + (self.elapsed * 2.0).sin() * 0.3;
                }
            }
            if t == 1.0 {
                self.values[spec.need] = (self.values[spec.need] + dt * 5.0).min(100.0);
            }
            if self.elapsed >= spec.duration {
                self.restore();
                self.task = None;
                self.phase = Phase::Idle;
                self.decision_in = 2.0;
                self.action = "Descansando".into();
            }
        }
        self.frame.drops = self.phase == Phase::Using && self.task == Some(Task::Shower);
        if self.frame.drops {
            for y in &mut self.drops_y {
                *y -= dt * 3.0;
                if *y < 0.3 {
                    *y = 2.5;
                }
            }
        }
        self.frame.watching = self.phase == Phase::Using && self.task == Some(Task::Tv);
        self.frame.screen_hue = (now / 14000.0) % 1.0;
        if self.ui_time > 0.3 {
            self.ui_time = 0.0;
            self.needs_visible = 5;
            self.displayed_values = self.values;
            let m = self.game_time.floor() as u64 % 1440;
            self.displayed_clock = format!("{:02}:{:02}", m / 60, m % 60);
        }
    }
}
