use std::collections::HashMap;
use tokio::sync::mpsc;

// 1. Declare sub-modules
pub mod bus;
pub mod host;
pub mod state_machine;
pub mod discovery;

use crate::hydrust::protocol::events::{Event, CoreEvent, BrowserRequest, StreamInfo};
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

    pub async fn handle_plugin_discovery(&mut self) {
        println!("Starting plugin discovery...");
        let plugins = discovery::discover_plugins();
        println!("Discovered {} plugins.", plugins.len());
        for p in plugins {
            println!("   - {} v{} by {}", p.name, p.version, p.author);
            self.plugins.push(LoadedPlugin { id: p.id });
        }
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