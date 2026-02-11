// crates/hydrust-core/src/engine/discovery.rs

use std::path::PathBuf;
use tokio::fs;
use crate::load_plugin_metadata;

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub path: PathBuf, // Keep track of where the file is for later loading
}

pub async fn scan_plugins_folder() -> Vec<PluginInfo> {
    let mut plugins = Vec::new();
    let plugins_path = PathBuf::from("../../plugins");

    // 1. Ensure directory exists
    if !plugins_path.exists() {
        let _ = fs::create_dir_all(&plugins_path).await;
        return plugins;
    }

    // 2. Read directory entries
    let mut entries = match fs::read_dir(plugins_path).await {
        Ok(e) => e,
        Err(_) => return plugins,
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        
        // 3. Only look for .wasm files
        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
            if let Ok(wasm_bytes) = fs::read(&path).await {
                // 4. Ask the runtime to extract metadata from the WASM component
                match load_plugin_metadata(&wasm_bytes).await {
                    Ok(metadata) => {
                        plugins.push(PluginInfo {
                            id: shared::generate_uuid(),
                            name: metadata.name,
                            version: metadata.version,
                            author: metadata.author,
                            description: metadata.description,
                            path: path.clone(),
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to load metadata from {:?}: {}", path, e);
                    }
                }
            }
        }
    }

    plugins
}