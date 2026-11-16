//! Fatal startup errors must remain visible in the Windows GUI executable.
use std::{backtrace::Backtrace, fs, sync::atomic::{AtomicBool, Ordering}};

static REPORTED: AtomicBool = AtomicBool::new(false);

pub fn install() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let details = format!(
            "mens-life {}\nThread: {}\nExecutable: {:?}\nWorking directory: {:?}\n\n{}\n\n{}\n",
            env!("CARGO_PKG_VERSION"), thread.name().unwrap_or("unnamed"),
            std::env::current_exe(), std::env::current_dir(), info, Backtrace::force_capture(),
        );
        report(&details);
        previous(info);
    }));
}

pub fn report(details: &str) {
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
    if std::env::args().any(|a| a == "--capture") { return; }
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
