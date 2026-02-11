//! The event bus system for the Hydrust Engine.
//!
//! This module handles the orchestration of messages between WebAssembly plugins,
//! external services, and the internal core engine logic.

use crate::engine::HydrustEngine;
use crate::hydrust::protocol::events::{CoreEvent, Event, EventPayload, ServiceEvent};
use tokio::sync::{ mpsc, broadcast };

/// Events that are restricted to internal core communication.
///
/// These events are used for host-side logic that does not need to be
/// exposed to the WebAssembly guest environment or the WIT interface.
#[derive(Clone, Debug)]
pub enum InternalCoreEvent {
    /// Signals the engine to perform a discovery process for plugins.
    PluginDiscovery,
    /// Indicates that the plugin discovery process has completed.
    PluginDiscoveryComplete(Vec<crate::engine::discovery::PluginInfo>),
}

/// A wrapper for unifying different event categories into a single stream.
///
/// This facilitates a single processing loop that can handle both
/// WIT-generated external events and host-only internal events.
#[derive(Clone, Debug)]
pub enum BusMessage {
    /// An event generated via the WIT host interface or external sources.
    External(Event),
    /// An event generated internally for core-to-core communication.
    Internal(InternalCoreEvent),
}

/// The central communication hub responsible for routing and dispatching events.
///
/// It maintains a receiver channel to process incoming messages and routes them
/// to the appropriate handler methods on the engine.
pub struct EventBus {
    /// The sender half of the event channel.
    tx: mpsc::Sender<BusMessage>,
    /// The receiver half of the event channel.
    rx: mpsc::Receiver<BusMessage>,
    notifier: broadcast::Sender<BusMessage>,
}

impl EventBus {
    /// Creates a new instance of the EventBus.
    ///
    /// Returns a tuple containing the EventBus itself and a Sender to
    /// dispatch events into the bus.
    pub fn new() -> (Self, mpsc::Sender<BusMessage>) {
        let (tx, rx) = mpsc::channel(100);
        // Create a broadcast channel with a buffer of 100 messages
        let (notifier, _) = broadcast::channel(100);
        
        (Self { tx: tx.clone(), rx, notifier }, tx)
    }

    /// Allows the TUI or other components to listen to the bus
    pub fn subscribe(&self) -> broadcast::Receiver<BusMessage> {
        self.notifier.subscribe()
    }

    /// Processes events originating from the WIT interface or plugins.
    ///
    /// This method routes external events to the appropriate engine handlers
    /// based on the payload type (Plugin, Core, or Service).
    async fn handle_external_event(&mut self, engine: &mut HydrustEngine, event: Event) {
        match event.payload {
            EventPayload::Plugin(p_ev) => match p_ev {
                _ => {}
            },
            EventPayload::Core(c_ev) => match c_ev {
                CoreEvent::IntentResolve(url) => {
                    todo!("Handle intent resolution if needed")
                }
                CoreEvent::BrowserObserved(data) => {
                    todo!("Handle browser observations if needed")
                }
                _ => {}
            },
            EventPayload::Service(s_ev) => match s_ev {
                ServiceEvent::MediaProgress(pct) => {
                    todo!("Handle media progress if needed")
                }
                ServiceEvent::MediaComplete(path) => {
                    todo!("Handle media completion if needed")
                }
            },
        }
    }

    /// Processes events originating from within the host core.
    ///
    /// These events facilitate communication between different parts of
    /// the host engine without involving the guest plugin environment.
    async fn handle_internal_event(
        &mut self,
        engine: &mut HydrustEngine,
        event: InternalCoreEvent,
    ) {
        match event {
            InternalCoreEvent::PluginDiscovery => {
                engine.handle_plugin_discovery().await;
            }
            InternalCoreEvent::PluginDiscoveryComplete(names) => {
                // Handle any post-discovery logic if needed
            }
        }
    }

    /// Starts the main event loop.
    ///
    /// This is a long running task that listens for incoming BusMessages
    /// and dispatches them to their respective internal or external handlers.
    pub async fn run(&mut self, engine: &mut HydrustEngine) {
        while let Some(msg) = self.rx.recv().await {
            // 1. Broadcast the message to all subscribers (like the TUI)
            // We ignore errors here because it's okay if no one is listening yet
            // TODO: Add logging for dropped messages if there are no subscribers and trace logging
            let _ = self.notifier.send(msg.clone());

            match msg {
                BusMessage::External(event) => {
                    self.handle_external_event(engine, event).await;
                }

                BusMessage::Internal(event) => {
                    self.handle_internal_event(engine, event).await;
                }
            }
        }
    }
}
