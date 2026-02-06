use crate::events::Event;
use tokio::sync::mpsc;

pub struct MyHostState {
    pub trace_id: String,
    pub event_bus_tx: mpsc::Sender<Event>,
}

impl crate::bindings::hydrust::protocol for MyHostState {
    fn publish(&mut self, ev: Event) -> anyhow::Result<()> {
        let mut event_to_route = ev;
        event_to_route.origin = "plugin".to_string(); 
        
        // Use try_send or blocking_send depending on your needs
        self.event_bus_tx.blocking_send(event_to_route)?;
        Ok(())
    }
}