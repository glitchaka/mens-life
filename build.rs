use std::{fs, path::Path};

const HOUSE_ASSET_BYTES: u64 = 1_110_460;

fn verify_house_asset() {
    let target = Path::new("assets/house/house.glb");
    println!("cargo:rerun-if-changed={}", target.display());

    let bytes = fs::read(target).expect(
        "PSX Backroom is missing from assets/house/house.glb. Run install.cmd (or tools/ensure-assets.ps1) before building.",
    );
    assert_eq!(
        bytes.len() as u64,
        HOUSE_ASSET_BYTES,
        "PSX Backroom package size mismatch",
    );
    assert!(bytes.starts_with(b"glTF"), "PSX Backroom package is not a GLB");
    assert!(bytes.len() >= 12, "PSX Backroom GLB header is truncated");
    let declared = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    assert_eq!(declared, bytes.len(), "PSX Backroom GLB length header mismatch");
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
    verify_house_asset();
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
