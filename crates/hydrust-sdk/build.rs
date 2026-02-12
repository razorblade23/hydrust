use std::env;
use std::fs;
use std::io::copy;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_wit_dir = manifest_dir.join("../../wit");
    let sdk_wit_dir = manifest_dir.join("wit");

    // --- PART 1: Sync Workspace (Local Dev Mode) ---
    if workspace_wit_dir.exists() {
        // We only clean and copy if we are in the monorepo
        // This keeps your provider.wit in sync with the Core
        let _ = fs::remove_dir_all(&sdk_wit_dir);
        fs::create_dir_all(&sdk_wit_dir)?;

        let options = fs_extra::dir::CopyOptions::new().content_only(true);
        fs_extra::dir::copy(&workspace_wit_dir, &sdk_wit_dir, &options)
            .expect("Failed to copy WIT files to SDK");
        
        println!("cargo:rerun-if-changed=../../wit");
    }

    // --- PART 2: Ensure WASI Dependencies (Independent Mode) ---
    // Even if we copied from workspace, the workspace might be missing deps.
    // This ensures wit/deps/io and wit/deps/clocks always exist.
    ensure_wasi_deps(&sdk_wit_dir)?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn ensure_wasi_deps(sdk_wit_dir: &Path) -> anyhow::Result<()> {
    let deps_to_download = [
        ("io", "poll.wit", "https://raw.githubusercontent.com/WebAssembly/wasi-io/main/wit/poll.wit"),
        ("io", "streams.wit", "https://raw.githubusercontent.com/WebAssembly/wasi-io/main/wit/streams.wit"),
        ("io", "error.wit", "https://raw.githubusercontent.com/WebAssembly/wasi-io/main/wit/error.wit"),
        ("clocks", "wall-clock.wit", "https://raw.githubusercontent.com/WebAssembly/wasi-clocks/main/wit/wall-clock.wit"),
    ];

    for (folder, filename, url) in deps_to_download {
        let dir_path = sdk_wit_dir.join("deps").join(folder);
        let file_path = dir_path.join(filename);

        if !file_path.exists() {
            fs::create_dir_all(&dir_path)?;
            let mut response = reqwest::blocking::get(url)?;
            let mut out = fs::File::create(&file_path)?;
            copy(&mut response, &mut out)?;
        }
    }
    Ok(())
}