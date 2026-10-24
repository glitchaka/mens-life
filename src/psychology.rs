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
}

#[repr(C)]
struct NativeOption {
    task: i32,
    cost: f32,
    progress: f32,
}

unsafe extern "C" {
    fn ml_brain_create(seed: u64, error: *mut u8, capacity: usize) -> *mut c_void;
    fn ml_brain_destroy(brain: *mut c_void);
    fn ml_brain_update(brain: *mut c_void, ticks: u64, values: *const f32) -> i32;
    fn ml_brain_decide(
        brain: *mut c_void,
        options: *const NativeOption,
        count: usize,
        current: i32,
        selected: *mut i32,
    ) -> i32;
    fn ml_brain_event(brain: *mut c_void, task: i32, kind: i32) -> i32;
    fn ml_brain_state(brain: *const c_void, output: *mut PsychState) -> i32;
    fn ml_brain_error(brain: *const c_void, output: *mut u8, capacity: usize);
}

#[derive(Debug)]
struct Handle(NonNull<c_void>);
// Each handle owns a separate runtime. Mutex prevents concurrent access from
// Bevy's systems; the pointer is never exposed or shared with native callbacks.
unsafe impl Send for Handle {}
impl Drop for Handle {
    fn drop(&mut self) {
        // SAFETY: this is the unique owner, allocated by ml_brain_create.
        unsafe { ml_brain_destroy(self.0.as_ptr()) };
    }
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
    pub fn new(seed: u64) -> Result<Self, String> {
        let mut error = [0u8; 512];
        // SAFETY: the buffer is writable for its advertised length. C++ catches
        // every exception at the ABI boundary and returns an owned opaque handle.
        let pointer = unsafe { ml_brain_create(seed, error.as_mut_ptr(), error.len()) };
        let handle = NonNull::new(pointer).ok_or_else(|| error_text(&error))?;
        let mut this = Self {
            handle: Mutex::new(Handle(handle)),
            state: PsychState::default(),
            error: None,
            seconds: 0.0,
        };
        this.read_state();
        Ok(this)
    }

    fn pointer(&mut self) -> *mut c_void {
        self.handle.get_mut().expect("exclusive MSPA handle").0.as_ptr()
    }

    fn finish_call(&mut self, pointer: *mut c_void, status: i32) -> bool {
        if status == 0 {
            let mut buffer = [0u8; 512];
            // SAFETY: live, exclusively borrowed runtime and writable buffer.
            unsafe { ml_brain_error(pointer, buffer.as_mut_ptr(), buffer.len()) };
            let message = error_text(&buffer);
            self.error = Some(if message.is_empty() {
                "MSPA no pudo procesar la operación".into()
            } else {
                message
            });
            false
        } else {
            self.error = None;
            self.read_state();
            true
        }
    }

    fn read_state(&mut self) {
        let pointer = self.pointer();
        // SAFETY: C++ writes exactly the repr(C) snapshot; no C++ containers cross FFI.
        unsafe { ml_brain_state(pointer, &mut self.state) };
    }

    pub fn advance(&mut self, dt: f64, values: [f64; 5]) {
        self.seconds += dt;
        let ticks = (self.seconds * 1000.0).round() as u64;
        let needs = values.map(|value| value.clamp(0.0, 100.0) as f32);
        let pointer = self.pointer();
        // SAFETY: needs contains five readable floats for the duration of the call.
        let status = unsafe { ml_brain_update(pointer, ticks, needs.as_ptr()) };
        self.finish_call(pointer, status);
    }

    pub fn decide(&mut self, available: &[(Task, f32, f32)], current: Option<Task>) -> Option<Task> {
        let mut options: Vec<_> = available
            .iter()
            .map(|(task, cost, progress)| NativeOption {
                task: task.spec().need as i32,
                cost: *cost,
                progress: *progress,
            })
            .collect();
        // Rest is a real host action, so healthy characters need not use furniture endlessly.
        options.push(NativeOption { task: -1, cost: 0.0, progress: 0.0 });
        let pointer = self.pointer();
        let mut selected = -1;
        // SAFETY: C++ only reads this live slice and writes one i32 selection.
        let status = unsafe {
            ml_brain_decide(
                pointer,
                options.as_ptr(),
                options.len(),
                current.map_or(-1, |task| task.spec().need as i32),
                &mut selected,
            )
        };
        if !self.finish_call(pointer, status) {
            return None;
        }
        available.iter().find_map(|(task, _, _)| {
            (task.spec().need as i32 == selected).then_some(*task)
        })
    }

    pub fn event(&mut self, task: Task, event: ActivityEvent) {
        let pointer = self.pointer();
        // SAFETY: task/event are closed Rust enums mapped to the documented C ABI.
        let status = unsafe { ml_brain_event(pointer, task.spec().need as i32, event as i32) };
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
            Self::Disgusted => ">_<", Self::Tired => "-_-", Self::Hungry => ":P",
            Self::Dirty => ":/", Self::Bored => ":|", Self::Urgent => "O_O",
            Self::Sleeping => "Zzz",
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
