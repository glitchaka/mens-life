//! Safe ownership of the real MSPA C++ runtime and presentation of its affect.
//! Physical needs, navigation and animation remain owned by the Rust game.
use crate::game::Task;
use std::{ffi::c_void, ptr::NonNull, sync::Mutex};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PsychState {
    pub emotions: [f32; 6],
    pub valence: f32,
    pub activation: f32,
    pub confidence: f32,
    pub memories: u32,
    pub beliefs: u32,
    pub learned_patterns: u32,
    pub personality: [f32; 6],
    pub derived_traits: [f32; 6],
}

#[repr(C)]
struct NativeOption {
    task: i32,
    cost: f32,
    progress: f32,
}

unsafe extern "C" {
    fn ml_brain_create_agent(seed: u64, agent: u64, error: *mut u8, capacity: usize) -> *mut c_void;
    fn ml_brain_destroy(brain: *mut c_void);
    fn ml_brain_update(brain: *mut c_void, ticks: u64, values: *const f32) -> i32;
    fn ml_brain_decide(brain: *mut c_void, options: *const NativeOption, count: usize,
        current: i32, selected: *mut i32) -> i32;
    fn ml_brain_event(brain: *mut c_void, task: i32, kind: i32) -> i32;
    fn ml_brain_environment(brain: *mut c_void, kind: i32, salience: f32) -> i32;
    fn ml_brain_state(brain: *const c_void, output: *mut PsychState) -> i32;
    fn ml_brain_error(brain: *const c_void, output: *mut u8, capacity: usize);
    fn ml_brain_social_decide(brain: *mut c_void, target: u64, selected: *mut i32) -> i32;
    fn ml_brain_social_report(brain: *mut c_void, target: u64, kind: i32, success: i32) -> i32;
    fn ml_brain_relation(brain: *const c_void, target: u64, output: *mut f32) -> i32;
}

#[derive(Debug)]
struct Handle(NonNull<c_void>);
unsafe impl Send for Handle {}
impl Drop for Handle {
    fn drop(&mut self) { unsafe { ml_brain_destroy(self.0.as_ptr()) }; }
}

#[derive(Debug)]
pub struct Psychology {
    handle: Mutex<Handle>,
    pub state: PsychState,
    pub error: Option<String>,
    seconds: f64,
}

fn error_text(buffer: &[u8]) -> String {
    let length = buffer.iter().position(|b| *b == 0).unwrap_or(buffer.len());
    String::from_utf8_lossy(&buffer[..length]).into_owned()
}

impl Psychology {
    pub fn new(seed: u64) -> Result<Self, String> { Self::new_agent(1, seed) }

    pub fn new_agent(agent: u64, seed: u64) -> Result<Self, String> {
        let mut error = [0u8; 512];
        let pointer = unsafe { ml_brain_create_agent(seed, agent, error.as_mut_ptr(), error.len()) };
        let handle = NonNull::new(pointer).ok_or_else(|| error_text(&error))?;
        let mut this = Self { handle: Mutex::new(Handle(handle)), state: PsychState::default(), error: None, seconds: 0.0 };
        this.read_state();
        Ok(this)
    }

    fn pointer(&mut self) -> *mut c_void {
        self.handle.get_mut().expect("exclusive MSPA handle").0.as_ptr()
    }

    fn finish_call(&mut self, pointer: *mut c_void, status: i32) -> bool {
        if status == 0 {
            let mut buffer = [0u8; 512];
            unsafe { ml_brain_error(pointer, buffer.as_mut_ptr(), buffer.len()) };
            let message = error_text(&buffer);
            self.error = Some(if message.is_empty() { "MSPA no pudo procesar la operación".into() } else { message });
            false
        } else {
            self.error = None;
            self.read_state();
            true
        }
    }

    fn read_state(&mut self) {
        let pointer = self.pointer();
        unsafe { ml_brain_state(pointer, &mut self.state) };
    }

    pub fn advance(&mut self, dt: f64, values: [f64; 5]) {
        self.seconds += dt;
        let ticks = (self.seconds * 1000.0).round() as u64;
        let needs = values.map(|value| value.clamp(0.0, 100.0) as f32);
        let pointer = self.pointer();
        let status = unsafe { ml_brain_update(pointer, ticks, needs.as_ptr()) };
        self.finish_call(pointer, status);
    }

    pub fn decide(&mut self, available: &[(Task, f32, f32)], current: Option<Task>) -> Option<Task> {
        let mut options: Vec<_> = available.iter().map(|(task, cost, progress)| NativeOption {
            task: task.spec().need as i32, cost: *cost, progress: *progress,
        }).collect();
        options.push(NativeOption { task: -1, cost: 0.0, progress: 0.0 });
        let pointer = self.pointer();
        let mut selected = -1;
        let status = unsafe {
            ml_brain_decide(pointer, options.as_ptr(), options.len(),
                current.map_or(-1, |task| task.spec().need as i32), &mut selected)
        };
        if !self.finish_call(pointer, status) { return None; }
        available.iter().find_map(|(task, _, _)|
            (task.spec().need as i32 == selected).then_some(*task))
    }

    pub fn event(&mut self, task: Task, event: ActivityEvent) {
        let pointer = self.pointer();
        let status = unsafe { ml_brain_event(pointer, task.spec().need as i32, event as i32) };
        self.finish_call(pointer, status);
    }

    /// Registers an anonymous environmental cue. The character is the perceiver,
    /// never the asserted cause, so this cannot reveal a player meta-identity.
    pub fn notice(&mut self, task: Task, salience: f32) {
        let pointer = self.pointer();
        let status = unsafe {
            ml_brain_environment(pointer, task.spec().need as i32, salience.clamp(0.0, 1.0))
        };
        self.finish_call(pointer, status);
    }

    pub fn emotion(&self) -> Expression {
        let (index, intensity) = self.state.emotions.iter().copied().enumerate()
            .max_by(|a, b| a.1.total_cmp(&b.1)).unwrap_or((0, 0.0));
        if intensity >= 0.18 {
            [Expression::Happy, Expression::Sad, Expression::Afraid,
             Expression::Angry, Expression::Anxious, Expression::Disgusted][index]
        } else if self.state.valence < -0.12 {
            Expression::Sad
        } else if self.state.valence > 0.12 {
            Expression::Happy
        } else {
            Expression::Calm
        }
    }

    pub fn social_choice(&mut self, target: u64) -> Option<SocialKind> {
        let pointer = self.pointer();
        let mut selected = -1;
        let status = unsafe { ml_brain_social_decide(pointer, target, &mut selected) };
        if !self.finish_call(pointer, status) { return None; }
        match selected {
            0 => Some(SocialKind::Greet), 1 => Some(SocialKind::Chat),
            2 => Some(SocialKind::Joke), _ => None,
        }
    }

    pub fn social_report(&mut self, target: u64, kind: SocialKind, success: bool) {
        let pointer = self.pointer();
        let status = unsafe { ml_brain_social_report(pointer, target, kind as i32, i32::from(success)) };
        self.finish_call(pointer, status);
    }

    pub fn relationship(&mut self, target: u64) -> Relationship {
        let pointer = self.pointer();
        let mut values = [0.0; 4];
        unsafe { ml_brain_relation(pointer, target, values.as_mut_ptr()) };
        Relationship { affection: values[0], trust: values[1], respect: values[2], familiarity: values[3] }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocialKind { Greet = 0, Chat = 1, Joke = 2 }

#[derive(Clone, Copy, Debug, Default)]
pub struct Relationship {
    pub affection: f32,
    pub trust: f32,
    pub respect: f32,
    pub familiarity: f32,
}
impl Relationship {
    pub fn label(self) -> &'static str {
        if self.affection < -0.15 { "Relación tensa" }
        else if self.familiarity >= 0.6 && self.affection >= 0.2 { "Amistad" }
        else if self.familiarity >= 0.2 { "Buena compañía" }
        else { "Se están conociendo" }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum ActivityEvent { Started = 0, Completed = 1, Failed = 2, Interrupted = 3 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Expression {
    Calm, Happy, Sad, Afraid, Angry, Anxious, Disgusted,
    Tired, Hungry, Dirty, Bored, Urgent, Sleeping,
}
impl Expression {
    pub fn emoticon(self) -> &'static str {
        match self {
            Self::Calm => ":-)", Self::Happy => ":D", Self::Sad => ":-(",
            Self::Afraid => ":O", Self::Angry => ">:(", Self::Anxious => ":S",
            Self::Disgusted => ">_<", Self::Tired => "-_ -".replace(' ', "").leak(),
            Self::Hungry => ":P", Self::Dirty => ":/", Self::Bored => ":|",
            Self::Urgent => "O_O", Self::Sleeping => "Zzz",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Calm => "Tranquilo", Self::Happy => "Contento", Self::Sad => "Desanimado",
            Self::Afraid => "Preocupado", Self::Angry => "Enfadado", Self::Anxious => "Inquieto",
            Self::Disgusted => "Incómodo", Self::Tired => "Cansado", Self::Hungry => "Hambriento",
            Self::Dirty => "Necesito una ducha", Self::Bored => "Aburrido",
            Self::Urgent => "¡Necesito el baño!", Self::Sleeping => "Soñando",
        }
    }

    pub fn words(self) -> &'static str {
        match self {
            Self::Calm => "Estoy a gusto en casa.",
            Self::Happy => "¡Esto me ha sentado bien!",
            Self::Sad => "Hoy me cuesta levantar el ánimo.",
            Self::Afraid => "Necesito ocuparme de esto pronto.",
            Self::Angry => "¡Qué rabia! Necesito un momento.",
            Self::Anxious => "No consigo estar del todo tranquilo.",
            Self::Disgusted => "Uf... esto no me resulta agradable.",
            Self::Tired => "Me pesan los párpados... necesito dormir.",
            Self::Hungry => "Me ruge el estómago. Quiero comer algo.",
            Self::Dirty => "Una ducha me vendría de maravilla.",
            Self::Bored => "Necesito distraerme un rato.",
            Self::Urgent => "¡Al baño, rápido!",
            Self::Sleeping => "Zzz... cinco minutitos más...",
        }
    }

    pub fn color(self) -> u32 {
        match self {
            Self::Happy | Self::Calm => 0x527f7e,
            Self::Angry | Self::Urgent => 0xc56861,
            Self::Afraid | Self::Anxious => 0xa47baf,
            Self::Sad | Self::Tired | Self::Sleeping => 0x6a89ae,
            Self::Disgusted | Self::Dirty => 0x7f9866,
            Self::Hungry | Self::Bored => 0xc08b42,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Reaction {
    pub expression: Expression,
    pub words: String,
    pub remaining: f64,
}
impl Default for Reaction {
    fn default() -> Self {
        Self {
            expression: Expression::Calm,
            words: "¡Vamos a estrenar esta casa!".into(),
            remaining: 7.0,
        }
    }
}
