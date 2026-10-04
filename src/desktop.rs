#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{borrow::Cow, fs, path::PathBuf};
    use tao::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use wry::{WebViewBuilder, http::Response};

    // Every asset, including Wasm and the renderer, is built into this executable.
    // No server, Node installation or connection to Sites is needed at runtime.
    let assets: &[(&str, &str, &[u8])] = &[
        (
            "/",
            "text/html; charset=utf-8",
            include_bytes!("../dist/index.html"),
        ),
        (
            "/index.html",
            "text/html; charset=utf-8",
            include_bytes!("../dist/index.html"),
        ),
        (
            "/styles.css",
            "text/css; charset=utf-8",
            include_bytes!("../dist/styles.css"),
        ),
        (
            "/preflight.css",
            "text/css; charset=utf-8",
            include_bytes!("../dist/preflight.css"),
        ),
        (
            "/favicon.svg",
            "image/svg+xml",
            include_bytes!("../dist/favicon.svg"),
        ),
        (
            "/bridge.js",
            "text/javascript; charset=utf-8",
            include_bytes!("../dist/bridge.js"),
        ),
        (
            "/pkg/mens_life.js",
            "text/javascript; charset=utf-8",
            include_bytes!("../dist/pkg/mens_life.js"),
        ),
        (
            "/pkg/mens_life_bg.wasm",
            "application/wasm",
            include_bytes!("../dist/pkg/mens_life_bg.wasm"),
        ),
        (
            "/vendor/three.module.js",
            "text/javascript; charset=utf-8",
            include_bytes!("../dist/vendor/three.module.js"),
        ),
        (
            "/vendor/three.core.js",
            "text/javascript; charset=utf-8",
            include_bytes!("../dist/vendor/three.core.js"),
        ),
        (
            "/vendor/RoundedBoxGeometry.js",
            "text/javascript; charset=utf-8",
            include_bytes!("../dist/vendor/RoundedBoxGeometry.js"),
        ),
    ];
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vida Isométrica 3D")
        .with_inner_size(LogicalSize::new(1280.0, 800.0))
        .build(&event_loop)?;
    let data_directory = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("MensLife")
        .join("WebView2");
    fs::create_dir_all(&data_directory)?;
    let mut context = wry::WebContext::new(Some(data_directory));
    let webview = WebViewBuilder::new_with_web_context(&mut context)
        .with_custom_protocol("menslife".into(), move |_id, request| {
            let path = request.uri().path();
            match assets.iter().find(|(name, _, _)| *name == path) {
                Some((_, mime, bytes)) => Response::builder()
                    .header("Content-Type", *mime)
                    .header("Cache-Control", "no-cache")
                    .body(Cow::Borrowed(*bytes))
                    .unwrap(),
                None => Response::builder()
                    .status(404)
                    .header("Content-Type", "text/plain")
                    .body(Cow::Borrowed(b"Not found".as_slice()))
                    .unwrap(),
            }
        })
        .with_url("menslife://localhost/")
        .build(&window)?;
    event_loop.run(move |event, _, flow| {
        let _ = &webview;
        let _ = &context;
        let _ = &window;
        *flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *flow = ControlFlow::Exit;
        }
    });
}

#[cfg(not(windows))]
fn main() {
    eprintln!(
        "El ejecutable de escritorio requiere Windows. Usa la compilación WebAssembly en otros sistemas: npm run build."
    );
    std::process::exit(1);
}
