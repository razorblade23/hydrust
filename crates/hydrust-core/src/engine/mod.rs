use std::collections::HashMap;
use tokio::sync::mpsc;

// 1. Declare sub-modules
pub mod bus;
pub mod host;
pub mod state_machine; 

use crate::events::{Event, Payload, CoreEvent, BrowserRequest, StreamInfo};
use crate::services::browser::BrowserService;
use self::state_machine::{DownloadTask, TaskState}; // Import the missing types

pub struct LoadedPlugin {
    pub id: String,
}

pub struct HydrustEngine {
    active_tasks: HashMap<String, DownloadTask>,
    plugins: Vec<LoadedPlugin>,
    bus_tx: mpsc::Sender<Event>,
    browser: BrowserService,
}

impl HydrustEngine {
    pub fn new(bus_tx: mpsc::Sender<Event>) -> Self {
        Self {
            active_tasks: HashMap::new(),
            plugins: Vec::new(),
            bus_tx,
            browser: BrowserService::new(),
        }
    }

    // --- HANDLERS ---

    pub async fn handle_browser_request(&mut self, trace_id: String, req: BrowserRequest) {
        if let Some(task) = self.active_tasks.get_mut(&trace_id) {
            task.current_state = TaskState::Interacting;
        }

        let tx = self.bus_tx.clone();
        // BrowserService is now Clone (see step 4)
        let mut browser = self.browser.clone(); 
        
        tokio::spawn(async move {
            match browser.sniff(req).await {
                Ok(sniffed_data) => {
                    let response = Event {
                        id: trace_id,
                        origin: "core".into(),
                        timestamp: 0,
                        payload: Payload::Core(CoreEvent::BrowserObserved(sniffed_data)),
                    };
                    let _ = tx.send(response).await;
                }
                Err(e) => eprintln!("Browser failed: {}", e),
            }
        });
    }

    pub async fn handle_plugin_identified(&mut self, trace_id: String, plugin_id: String) {
        println!("✅ Plugin [{}] claimed task [{}]", plugin_id, trace_id);
        if let Some(task) = self.active_tasks.get_mut(&trace_id) {
            task.assigned_plugin = Some(plugin_id);
            task.current_state = TaskState::Interacting;
        }
    }

    pub async fn handle_stream_ready(&mut self, trace_id: String, info: StreamInfo) {
        println!("🎉 STREAM FOUND for [{}]: {}", trace_id, info.title);
        // Trigger download/muxing here
    }

    pub async fn broadcast_to_plugins(&mut self, _event: Event) {
        println!("📣 Broadcasting to {} plugins...", self.plugins.len());
    }

    pub async fn forward_to_active_plugin(&mut self, event: Event) {
        if let Some(task) = self.active_tasks.get(&event.id) {
            if let Some(plugin_id) = &task.assigned_plugin {
                println!("Forwarding to plugin: {}", plugin_id);
            }
        }
    }
    
    // Add this to fix the download finished call in bus.rs
    pub async fn handle_download_finished(&mut self, path: String) {
        println!("💾 Download finished at: {}", path);
    }
}