use hydrust_sdk::{register_plugin, Handler};
use hydrust_sdk::events::Event;
use hydrust_sdk::metadata::PluginInfo;

#[derive(Default)]
struct MyPlugin;

impl Handler for MyPlugin {
    fn metadata(&self) -> PluginInfo {
        PluginInfo {
            name: "My Plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "razorblade23".to_string(),
            description: "My plugin does wonders".to_string()
        }
    }

    fn on_event(&self, event: Event) {
        // Handle event logic here
    }
}

register_plugin!(MyPlugin);