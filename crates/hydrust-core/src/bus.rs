use crate::engine::HydrustEngine;
use crate::hydrust::protocol::events::{Event, EventPayload, CoreEvent, ServiceEvent}; // Adjust imports based on your protocol
use crate::engine::discovery::PluginInfo; // Make sure this struct is public!
use tokio::sync::{mpsc, broadcast};

#[derive(Clone, Debug)]
pub enum InternalCoreEvent {
    PluginDiscovery,
    // We pass the full struct so the TUI has data to render
    PluginDiscoveryComplete(Vec<PluginInfo>), 
}

#[derive(Clone, Debug)]
pub enum BusMessage {
    External(Event),
    Internal(InternalCoreEvent),
}

pub struct EventBus {
    tx: mpsc::Sender<BusMessage>,
    rx: mpsc::Receiver<BusMessage>,
    // The "Radio Station" for the TUI
    pub notifier: broadcast::Sender<BusMessage>,
}

impl EventBus {
    pub fn new() -> (Self, mpsc::Sender<BusMessage>) {
        let (tx, rx) = mpsc::channel(100);
        let (notifier, _) = broadcast::channel(100);
        (Self { tx: tx.clone(), rx, notifier }, tx)
    }

    /// TUI calls this to start listening
    pub fn subscribe(&self) -> broadcast::Receiver<BusMessage> {
        self.notifier.subscribe()
    }

    pub async fn run(&mut self, engine: &mut HydrustEngine) {
        while let Some(msg) = self.rx.recv().await {
            // 1. Broadcast to TUI (ignore error if TUI isn't open yet)
            let _ = self.notifier.send(msg.clone());

            // 2. Handle Internally
            match msg {
                BusMessage::External(event) => {
                    // self.handle_external_event(engine, event).await;
                }
                BusMessage::Internal(event) => {
                     match event {
                        InternalCoreEvent::PluginDiscovery => {
                            engine.handle_plugin_discovery().await;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}