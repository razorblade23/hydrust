use crate::app::App;
use hydrust_core::engine::HydrustEngine;
use hydrust_core::bus::{ EventBus, BusMessage, InternalCoreEvent };
pub mod app;
pub mod event;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let (mut event_bus, bus_sender) = EventBus::new();
    let mut hydrust_engine = HydrustEngine::new(bus_sender.clone());
    
    // Get a dedicated ear for the TUI before moving the bus into the task
    let tui_rx = event_bus.subscribe();

    tokio::spawn(async move { event_bus.run(&mut hydrust_engine).await });

    // Trigger discovery - the TUI will now "hear" this happen!
    bus_sender.send(BusMessage::Internal(InternalCoreEvent::PluginDiscovery)).await?;

    let terminal = ratatui::init();
    // Pass the receiver and the sender to the App
    let result = App::new(bus_sender, tui_rx).run(terminal).await;
    
    ratatui::restore();
    result
}