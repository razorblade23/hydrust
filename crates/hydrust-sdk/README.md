# 💧 Hydrust SDK
The *official* Software Development Kit for building **Hydrust** WASM plugins.

Hydrust is a high-performance, plugin-based media framework. This SDK allows you to build site-specific logic (URL parsing, metadata extraction, stream discovery) that runs in a secure, sandboxed WebAssembly environment.

## 🚀 Quick Start
1. Create a new project
Plugins must be compiled as WebAssembly components.
```bash
cargo new --lib my-hydrust-plugin
cd my-hydrust-plugin
```

2. Configure Cargo.toml
Set the crate type to cdylib and add the SDK:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
hydrust-sdk = "0.1.0"
```

3. Implement the Plugin
Edit src/lib.rs to handle events from the Hydrust Core:

```rust
use hydrust_sdk::{register_plugin, Handler, events::*, metadata::PluginInfo};

#[derive(Default)]
struct YoutubePlugin;

impl Handler for YoutubePlugin {
    // This function is required
    fn metadata(&self) -> PluginInfo {
        PluginInfo {
            name: "MyHydrustPlugin".to_string(),
            version: "0.1.0".to_string(),
            author: Some("MyName".to_string()),
            description: Some("My first Hydrust plugin".to_string())
        }
    }

    fn on_event(&self, ev: hydrust_sdk::events::Event) {
        let _trace_id = ev.id.clone(); // This is required to trace the events across the system
        
        if let EventPayload::Core(CoreEvent::IntentResolve(url)) = ev.payload {
             if url.contains("youtube.com") {
                 // Logic...
             }
        }
    }
}

register_plugin!(YoutubePlugin);
```

## 🛠 Features Included
- Strongly Typed Events: Full access to the Hydrust WIT protocol.
- HTML Parsing: Includes scraper for efficient DOM manipulation inside WASM.
- JSON Support: serde and serde_json ready for API interaction.
- Sandboxed Execution: Your plugin runs in a secure environment with managed access to the host.

## 📦 Building your Plugin
To compile your plugin to a WASM component, we recommend using `cargo component`:
```bash
cargo component build --release
```
The resulting `.wasm` file in *target/wasm32-wasip1/release/* can be dropped directly into the **Hydrust** *plugins/* directory.

## 📜 License
Licensed under **MIT** license.