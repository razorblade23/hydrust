use crate::events::{Event, EventPayload};
use tokio::sync::mpsc;

// This struct holds the state for each WASM instance
pub struct MyHostState {
    pub trace_id: String,
    pub event_bus_tx: mpsc::Sender<Event>,
}

// We implement the generated Host trait
impl hydrust::protocol::site_provider::Host for MyHostState {
    fn publish(&mut self, ev: Event) -> anyhow::Result<()> {
        // We take the event from the plugin and drop it onto our central Bus
        let mut event_to_route = ev;
        event_to_route.origin = "plugin".to_string(); // Ensure origin is tracked
        
        self.event_bus_tx.blocking_send(event_to_route)?;
        Ok(())
    }
}