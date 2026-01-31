pub mod engine;
pub mod services;

// 1. Generate the Host bindings
mod bindings {
    wit_bindgen::generate!({
        path: "../../wit",
        world: "site-provider",
    });
}

// 2. Map the generated types to the 'events' module correctly
pub mod events {
    // We go inside the generated 'events' interface module
    // and pull out the 'Event' struct and the variants.
    pub use crate::bindings::hydrust::protocol::events::{
        Event, 
        EventPayload, 
        PluginEvent, 
        CoreEvent, 
        ServiceEvent,
        BrowserRequest,
        StreamInfo
    };

    // This satisfies your "use crate::events::Payload" imports
    pub type Payload = EventPayload;
}