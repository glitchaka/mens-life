#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
mod native;
mod startup;
fn main() -> std::process::ExitCode {
    startup::install();
    match native::run() {
        bevy::app::AppExit::Success => {
            startup::checkpoint("Bevy devolvió salida normal");
            std::process::ExitCode::SUCCESS
        },
        bevy::app::AppExit::Error(code) => {
            startup::checkpoint(&format!("Bevy devolvió error {}", code));
            startup::report(&format!("Bevy terminó con el código de error {}.", code));
            std::process::ExitCode::from(code.get())
        }
    }
}
