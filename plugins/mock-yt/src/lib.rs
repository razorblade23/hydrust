use hydrust_sdk::{register_plugin, Handler, events::*, metadata::PluginInfo};

#[derive(Default)]
struct YoutubePlugin;

impl Handler for YoutubePlugin {
    fn metadata(&self) -> PluginInfo {
        PluginInfo {
            name: "Youtube Provider".to_string(),
            version: "0.1.0".to_string(),
            author: Some("razorblade23".to_string()),
            description: Some("Mock plugin to test out SDK".to_string())
        }
    }

    fn on_event(&self, ev: hydrust_sdk::events::Event) {
        let _trace_id = ev.id.clone();
        
        // Note: You might need to import EventPayload, CoreEvent explicitly
        // or access them via ev.payload (if enum variants are imported)
        if let EventPayload::Core(CoreEvent::IntentResolve(url)) = ev.payload {
             if url.contains("youtube.com") {
                 // Logic...
             }
        }
    }
}

register_plugin!(YoutubePlugin);