pub mod engine;
pub mod services;

// 1. Generate the Host bindings from WIT
// We use the same WIT file, but we are the HOST now.
wit_bindgen::generate!({
    path: "../../wit",
    world: "site-provider",
    // This tells bindgen we are implementing the host side
    runtime_path: "wit_bindgen::rt", 
});

// 2. Create the 'events' module by re-exporting generated types
// This fixes "file not found for module events"
pub mod events {
    // Re-export the types so we can access them as crate::events::Event
    pub use crate::hydrust::protocol::events::*;
    
    // Also re-export the Payload/Event variants for easier access
    pub use crate::hydrust::protocol::events::EventPayload as Payload;
    pub use crate::hydrust::protocol::events::PluginEvent;
    pub use crate::hydrust::protocol::events::CoreEvent;
    pub use crate::hydrust::protocol::events::ServiceEvent;
}

use tokio::sync::mpsc;
use engine::{HydrustEngine, bus::EventBus};

pub async fn start_engine() -> anyhow::Result<()> {
    let (mut bus, bus_tx) = EventBus::new();
    
    // Explicitly define the channel type to help Rust inference
    let mut engine = HydrustEngine::new(bus_tx);

    println!("🚀 Hydrust Core Started. Waiting for events...");

    bus.run(&mut engine).await;

    Ok(())
}