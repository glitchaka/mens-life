use std::{fs, path::Path, process::Command};

const HOUSE_ASSET_BYTES: u64 = 1_110_460;

fn validate_house_asset(bytes: &[u8]) -> Result<(), &'static str> {
    if bytes.len() as u64 != HOUSE_ASSET_BYTES {
        return Err("PSX Backroom package size mismatch");
    }
    if !bytes.starts_with(b"glTF") {
        return Err("PSX Backroom package is not a GLB");
    }
    if u32::from_le_bytes(bytes[4..8].try_into().unwrap()) != 2 {
        return Err("PSX Backroom must use GLB version 2");
    }
    let declared = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    if declared != bytes.len() {
        return Err("PSX Backroom GLB length header mismatch");
    }
    Ok(())
}

fn verify_house_asset() {
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let target = root.join("assets/house/house.glb");
    println!("cargo:rerun-if-changed=assets/house/house.glb");
    println!("cargo:rerun-if-changed=tools/ensure-assets.ps1");
    println!("cargo:rerun-if-env-changed=CARGO_NET_OFFLINE");

    // A clean checkout omits this licensed runtime asset. Direct Cargo builds
    // must prepare it just like install.cmd, before compiling the native bridge.
    if !target.is_file() {
        assert!(
            std::env::var("CARGO_NET_OFFLINE").as_deref() != Ok("true"),
            "PSX Backroom is missing. Restore assets/house/house.glb or build once with network access before using CARGO_NET_OFFLINE=true.",
        );
        println!("cargo:warning=Preparing missing PSX Backroom runtime asset...");
        let output = if cfg!(windows) {
            Command::new("powershell.exe")
                .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"])
                .arg(root.join("tools/ensure-assets.ps1"))
                .current_dir(&root)
                .output()
                .expect("Cannot start PowerShell to prepare assets/house/house.glb")
        } else {
            fs::create_dir_all(target.parent().unwrap()).expect("Cannot create house asset directory");
            let temporary = root.join("assets/house/house.glb.download");
            let output = Command::new("curl")
                .args(["--fail", "--location", "--connect-timeout", "20", "--max-time", "180", "--output"])
                .arg(&temporary)
                .arg("https://raw.githubusercontent.com/crazy0codes/hollow-ward/bfed478dd712872cdfe764c4c669a2a8616dea67/public/assets/psx_-_backroom.glb")
                .output()
                .expect("curl is required to prepare the missing house asset");
            if output.status.success() {
                let bytes = fs::read(&temporary).expect("Cannot read downloaded house asset");
                if let Err(error) = validate_house_asset(&bytes) {
                    let _ = fs::remove_file(&temporary);
                    panic!("Downloaded house asset is invalid: {error}");
                }
                fs::rename(&temporary, &target).expect("Cannot install downloaded house asset");
            } else {
                let _ = fs::remove_file(&temporary);
            }
            output
        };
        assert!(output.status.success(),
            "Cannot prepare PSX Backroom. Check network access to raw.githubusercontent.com or restore assets/house/house.glb.\n{}\n{}",
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    }
    let bytes = fs::read(&target).expect("Cannot read assets/house/house.glb after asset preparation");
    validate_house_asset(&bytes).unwrap_or_else(|error| panic!("{error}: {}", target.display()));
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
