use std::collections::HashMap;
use tokio::sync::mpsc;

// 1. Declare sub-modules
pub mod discovery;
pub mod state_machine;

use self::state_machine::{DownloadTask, TaskState}; // Import the missing types
use crate::bus::{ BusMessage, InternalCoreEvent };
use crate::hydrust::protocol::events::BrowserRequest;
use crate::services::browser::BrowserService;

pub struct LoadedPlugin {
    pub id: String,
}

pub struct HydrustEngine {
    active_tasks: HashMap<String, DownloadTask>,
    plugins: Vec<discovery::PluginInfo>,
    bus_tx: mpsc::Sender<BusMessage>,
    browser: BrowserService,
}

impl HydrustEngine {
    pub fn new(bus_tx: mpsc::Sender<BusMessage>) -> Self {
        Self {
            active_tasks: HashMap::new(),
            plugins: vec![],
            bus_tx,
            browser: BrowserService::new(),
        }
    }

    // --- HANDLERS ---

    pub async fn handle_plugin_discovery(&mut self) {
        // Run the real scan
        let discovered = discovery::scan_plugins_folder().await;
        
        // Notify the bus (and TUI)
        let _ = self.bus_tx.send(BusMessage::Internal(
            InternalCoreEvent::PluginDiscoveryComplete(discovered)
        )).await;
    }

    pub async fn handle_browser_request(&mut self, trace_id: String, req: BrowserRequest) {
        if let Some(task) = self.active_tasks.get_mut(&trace_id) {
            task.current_state = TaskState::Interacting;
        }

        // let tx = self.bus_tx.clone();
        // let mut browser = self.browser.clone();

        // tokio::spawn(async move {
        //     match browser.sniff(req).await {
        //         Ok(sniffed_data) => {
        //             let response = Event {
        //                 id: trace_id,
        //                 origin: "core".into(),
        //                 timestamp: 0,
        //                 payload: crate::hydrust::protocol::events::EventPayload::Core(CoreEvent::BrowserObserved(sniffed_data)),
        //             };
        //             let _ = tx.send(response).await;
        //         }
        //         Err(e) => eprintln!("Browser failed: {}", e),
        //     }
        // });
    }
}
