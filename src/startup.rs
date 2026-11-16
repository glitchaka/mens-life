//! Fatal startup errors must remain visible in the Windows GUI executable.
use std::{backtrace::Backtrace, fs, io::Write, sync::atomic::{AtomicBool, Ordering}, time::{SystemTime, UNIX_EPOCH}};

static REPORTED: AtomicBool = AtomicBool::new(false);

/// Records lifecycle checkpoints even when a native/graphics failure bypasses Rust's panic hook.
pub fn checkpoint(event: &str) {
    let Some(dir) = std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_owned())) else { return; };
    let path = dir.join("last-run.txt");
    let mut opts = fs::OpenOptions::new();
    opts.create(true).write(true);
    if event == "Iniciando" { opts.truncate(true); } else { opts.append(true); }
    if let Ok(mut file) = opts.open(path) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let _ = writeln!(file, "[{now}] pid={} {event}", std::process::id());
    }
}

pub fn install() {
    checkpoint("Iniciando");
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let details = format!(
            "mens-life {}\nThread: {}\nExecutable: {:?}\nWorking directory: {:?}\n\n{}\n\n{}\n",
            env!("CARGO_PKG_VERSION"), thread.name().unwrap_or("unnamed"),
            std::env::current_exe(), std::env::current_dir(), info, Backtrace::force_capture(),
        );
        checkpoint("Pánico de Rust");
        report(&details);
        previous(info);
    }));
}

pub fn report(details: &str) {
    checkpoint("Error fatal");
    if REPORTED.swap(true, Ordering::SeqCst) { return; }
    // Keep reports inside the installation, independently of the launch directory.
    // If that directory is not writable, show the error without writing elsewhere.
    let saved = std::env::current_exe().ok().and_then(|exe| {
        let path = exe.parent()?.join("last-crash.txt");
        fs::write(&path, details).ok()?;
        Some(path)
    });
    let location = saved.map(|p| format!("Informe guardado en:\n{}", p.display()))
        .unwrap_or_else(|| "No se pudo guardar el informe del error.".into());
    let message = format!("El juego encontró un error fatal.\n\n{location}\n\n{details}");
    eprintln!("{message}");
    // Automated capture runs must fail without waiting for a modal dialog.
    if std::env::args().any(|a| a == "--capture" || a == "--smoke") { return; }
    show_error(&message);
}

#[cfg(windows)]
fn show_error(message: &str) {
    #[link(name = "user32")]
    unsafe extern "system" {
        fn MessageBoxW(window: *mut std::ffi::c_void, text: *const u16, caption: *const u16, flags: u32) -> i32;
    }
    let text: Vec<u16> = message.chars().take(2400).collect::<String>().encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = "mens-life — Error de ejecución".encode_utf16().chain(Some(0)).collect();
    unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), 0x10); }
}

#[cfg(not(windows))]
fn show_error(_: &str) {}
