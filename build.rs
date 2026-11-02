use std::{fs, path::{Path, PathBuf}};

const HOUSE_ASSET_BYTES: usize = 1_110_456;

fn decode_base64(input: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() * 3 / 4);
    let mut acc = 0u32;
    let mut bits = 0u32;
    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            b'\r' | b'\n' | b' ' | b'\t' => continue,
            _ => panic!("invalid base64 byte in house asset source"),
        } as u32;
        acc = (acc << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((acc >> bits) & 0xff) as u8);
        }
    }
    out
}

fn materialize_house_asset() {
    let source_dir = Path::new("assets/house/source");
    let target = Path::new("assets/house/house.glb");
    println!("cargo:rerun-if-changed={}", source_dir.display());

    let mut chunks = fs::read_dir(source_dir)
        .expect("PSX Backroom source chunks missing")
        .map(|entry| entry.expect("Cannot read house asset source").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "b64"))
        .collect::<Vec<PathBuf>>();
    chunks.sort();
    assert!(!chunks.is_empty(), "PSX Backroom source chunks missing");

    let mut encoded = String::new();
    for path in chunks {
        println!("cargo:rerun-if-changed={}", path.display());
        encoded.push_str(&fs::read_to_string(&path).expect("Cannot read house asset chunk"));
    }
    let decoded = decode_base64(&encoded);
    assert_eq!(decoded.len(), HOUSE_ASSET_BYTES, "PSX Backroom asset size mismatch");
    assert!(decoded.starts_with(b"glTF"), "PSX Backroom asset is not a GLB");
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("Cannot create house asset directory");
    }
    fs::write(target, decoded).expect("Cannot materialize PSX Backroom house asset");
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
    materialize_house_asset();
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
