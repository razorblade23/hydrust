pub use wit_bindgen;

pub mod rt {
    pub use wit_bindgen::rt::*;
}

// 1. Generate the bindings inside the SDK crate
wit_bindgen::generate!({
    path: "../../wit", 
    world: "site-provider",
    runtime_path: "crate::rt",
});


// The generated path follows the package name: hydrust:protocol -> crate::hydrust::protocol
pub mod events {
    pub use crate::hydrust::protocol::events::*;
}

pub mod types {
    pub use crate::hydrust::protocol::types::*;
}

/// The trait that every Hydrust extension must implement.
pub trait Handler {
    fn on_event(&self, event: events::Event);
}

// 3. The macro now just handles the boilerplate of the 'Guest' trait
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ident) => {
        // We still need the alias for wit-bindgen internal macros
        extern crate hydrust_sdk as wit_bindgen;

        struct GuestImpl;

        impl $crate::exports::hydrust::protocol::site_provider::Guest for GuestImpl {
            fn on_event(event: $crate::events::Event) {
                // Instantiate the user's handler and pass the event
                let plugin = $plugin_type;
                plugin.on_event(event);
            }
        }

        $crate::export!(GuestImpl);
    };
}

/// Helper to publish events back to the Host
pub fn send_event(id: String, payload: events::EventPayload) {
    crate::publish(&events::Event {
        id,
        origin: "plugin".into(),
        timestamp: 0, // Host can fill this on arrival
        payload,
    });
}