use tokio::sync::mpsc;
use crate::events::{Event, Payload, PluginEvent, CoreEvent, ServiceEvent};
use super::HydrustEngine; // Import the engine struct

pub struct EventBus {
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
}

impl EventBus {
    pub fn new() -> (Self, mpsc::Sender<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx: tx.clone(), rx }, tx)
    }

    pub async fn run(&mut self, engine: &mut HydrustEngine) {
        while let Some(event) = self.rx.recv().await {
            log::trace!("[{}] {} -> {:?}", event.id, event.origin, event.payload);
            
            match event.payload {
                // --- PLUGIN ROUTING ---
                Payload::Plugin(p_ev) => match p_ev {
                    PluginEvent::Identified(id) => engine.handle_plugin_identified(event.id, id).await,
                    PluginEvent::RequestBrowser(req) => engine.handle_browser_request(event.id, req).await,
                    PluginEvent::ResultReady(info) => engine.handle_stream_ready(event.id, info).await,
                    _ => {}
                },

                // --- CORE ROUTING ---
                Payload::Core(c_ev) => match c_ev {
                    CoreEvent::IntentResolve(url) => engine.broadcast_to_plugins(event).await,
                    CoreEvent::BrowserObserved(data) => engine.forward_to_active_plugin(event).await,
                    _ => {}
                },

                // --- SERVICE ROUTING ---
                Payload::Service(s_ev) => match s_ev {
                    ServiceEvent::MediaProgress(pct) => engine.ui.update_progress(pct),
                    ServiceEvent::MediaComplete(path) => engine.handle_download_finished(path).await,
                },
            }
        }
    }
}