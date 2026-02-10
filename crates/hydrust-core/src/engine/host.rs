use crate::{SiteProviderImports, hydrust::protocol::events::Event};
use tokio::sync::mpsc;

pub struct MyHostState {
    pub trace_id: String,
    pub event_bus_tx: mpsc::Sender<Event>,
}

impl SiteProviderImports for MyHostState {
    fn publish(&mut self, ev: Event) -> () {
        let mut event_to_route = ev;
        event_to_route.origin = "plugin".to_string(); 
        
        // Use try_send or blocking_send depending on your needs
        self.event_bus_tx.blocking_send(event_to_route).unwrap();
        ()
    }
}