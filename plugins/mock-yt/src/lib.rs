use hydrust_sdk::{register_plugin, Handler, events::*, types::*};

struct YoutubePlugin;

impl Handler for YoutubePlugin {
    fn on_event(&self, ev: Event) {
        // Trace ID is preserved throughout the conversation
        let trace_id = ev.id.clone();

        match ev.payload {
            // Namespace: Core
            EventPayload::Core(CoreEvent::IntentResolve(url)) => {
                if url.contains("youtube.com") {
                    hydrust_sdk::publish(trace_id, EventPayload::Plugin(
                        PluginEvent::Identified("hydrust.youtube".into())
                    ));
                }
            }
            
            // Namespace: Service (e.g., Browser)
            EventPayload::Core(CoreEvent::BrowserObserved(data)) => {
                // Logic to finish and send ResultReady...
            }

            _ => {}
        }
    }
}

register_plugin!(YoutubePlugin);