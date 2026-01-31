pub use wit_bindgen;

// 1. Define runtime explicitly at the crate root
pub mod rt {
    pub use wit_bindgen::rt::*;
}

// 2. Make bindings public (but hide from docs)
#[doc(hidden)]
pub mod bindings {
    use super::wit_bindgen;

    wit_bindgen::generate!({
        path: "../../wit", 
        world: "site-provider",
        runtime_path: "crate::rt", 
        pub_export_macro: true,
    });
}

// --- RE-EXPORTS ---

pub use bindings::Guest;

// Re-export types
pub mod events {
    pub use crate::bindings::hydrust::protocol::events::*;
}

pub mod types {
    pub use crate::bindings::hydrust::protocol::types::*;
}

pub use bindings::publish;

// --- USER FACING MACRO ---

pub trait Handler {
    fn on_event(&self, event: events::Event);
}

#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        struct GuestImpl;

        impl $crate::Guest for GuestImpl {
            fn on_event(event: $crate::events::Event) {
                let plugin: $plugin_type = Default::default();
                $crate::Handler::on_event(&plugin, event);
            }
        }

        $crate::bindings::export!(GuestImpl with_types_in $crate::bindings);
    };
}