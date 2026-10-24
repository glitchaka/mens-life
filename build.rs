use std::{fs, path::Path};

fn sources(directory: &Path, files: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(directory).expect("MSPA-Lib source directory missing") {
        let path = entry.expect("Cannot read MSPA-Lib source").path();
        if path.is_dir() {
            sources(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "cpp") {
            files.push(path);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=vendor/MSPA-Lib/include");
    println!("cargo:rerun-if-changed=vendor/MSPA-Lib/src");
    println!("cargo:rerun-if-changed=native/mspa_bridge.h");
    println!("cargo:rerun-if-changed=native/mspa_bridge.cpp");
    let mut files = Vec::new();
    sources(Path::new("vendor/MSPA-Lib/src"), &mut files);
    files.sort();
    files.push("native/mspa_bridge.cpp".into());
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include("vendor/MSPA-Lib/include")
        .include("native")
        .files(files);
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        build.flag("/EHsc").flag("/utf-8").flag("/fp:strict");
    } else {
        build.flag("-ffp-contract=off");
    }
    build.compile("mens_life_mspa");
}
