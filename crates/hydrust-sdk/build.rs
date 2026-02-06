use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // 1. Determine paths
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_wit_dir = manifest_dir.join("../../wit");
    let sdk_wit_dir = manifest_dir.join("wit");

    // 2. Only copy if the workspace wit directory exists
    // (This allows it to build on crates.io using the bundled copy)
    if workspace_wit_dir.exists() {
        // Clean old wit folder in SDK
        let _ = fs::remove_dir_all(&sdk_wit_dir);
        fs::create_dir_all(&sdk_wit_dir).unwrap();

        // Copy files from workspace root to SDK
        let options = fs_extra::dir::CopyOptions::new().content_only(true);
        fs_extra::dir::copy(&workspace_wit_dir, &sdk_wit_dir, &options)
            .expect("Failed to copy WIT files to SDK");
    }

    // 3. Tell Cargo to rerun this script if the original WIT changes
    println!("cargo:rerun-if-changed=../../wit");
}
