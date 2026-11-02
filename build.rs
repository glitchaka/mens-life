use std::{fs, path::{Path, PathBuf}};

const HOUSE_ASSET_BYTES: u64 = 1_110_456;
const HOUSE_NAMES: [&str; 3] = ["psx_-_backroom.glb", "psx-backroom.glb", "house.glb"];

fn valid_house_asset(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else { return false; };
    if metadata.len() != HOUSE_ASSET_BYTES { return false; }
    let Ok(bytes) = fs::read(path) else { return false; };
    bytes.starts_with(b"glTF")
}

fn house_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(explicit) = std::env::var("MENS_LIFE_HOUSE_GLB") {
        candidates.push(PathBuf::from(explicit));
    }
    if let Ok(cwd) = std::env::current_dir() {
        for name in HOUSE_NAMES { candidates.push(cwd.join(name)); }
    }
    for home_key in ["USERPROFILE", "HOME"] {
        if let Ok(home) = std::env::var(home_key) {
            let home = PathBuf::from(home);
            for directory in ["Downloads", "Desktop"] {
                for name in HOUSE_NAMES {
                    candidates.push(home.join(directory).join(name));
                }
            }
        }
    }
    candidates
}

fn install_house_asset() {
    let target = Path::new("assets/house/house.glb");
    println!("cargo:rerun-if-env-changed=MENS_LIFE_HOUSE_GLB");

    if valid_house_asset(target) {
        println!("cargo:rerun-if-changed={}", target.display());
        return;
    }
    if target.exists() {
        let _ = fs::remove_file(target);
    }

    for source in house_candidates() {
        println!("cargo:rerun-if-changed={}", source.display());
        if !valid_house_asset(&source) { continue; }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).expect("Cannot create assets/house");
        }
        fs::copy(&source, target).expect("Cannot install PSX Backroom house asset");
        println!("cargo:warning=Installed PSX Backroom from {}", source.display());
        return;
    }

    println!("cargo:warning=PSX Backroom not found. Put psx_-_backroom.glb in the repo root, Downloads, Desktop, or set MENS_LIFE_HOUSE_GLB.");
}

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
    install_house_asset();
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
