#![deny(missing_docs)]

//! # Hydrust Plugin SDK
//!
//! The official SDK for developing plugins for the Hydrust platform.
//!
//! This crate provides the necessary types, traits, and macros to define a guest
//! module that can be loaded by the Hydrust host. It abstracts away the low-level
//! `wit-bindgen` details and provides a clean, idiomatic Rust API.
//!
//! ## Usage
//!
//! To create a plugin, implement the [`Handler`] trait on a struct that derives
//! [`Default`], and then call [`register_plugin!`] with that struct type.
//!
//! ```rust,ignore
//! use hydrust_sdk::{register_plugin, Handler};
//! use hydrust_sdk::events::Event;
//! use hydrust_sdk::metadata::PluginInfo;
//!
//! #[derive(Default)]
//! struct MyPlugin;
//!
//! impl Handler for MyPlugin {
//!     fn metadata(&self) -> PluginInfo {
//!         PluginInfo {
//!             name: "My Plugin".to_string(),
//!             version: "0.1.0".to_string(),
//!             author: "MyName".to_string(),
//!             description: "My plugin does wonders".to_string()
//!         }
//!     }
//!
//!     fn on_event(&self, event: Event) {
//!         // Handle event logic here
//!     }
//! }
//!
//! register_plugin!(MyPlugin);
//! ```

/// Low-level access to the underlying `wit_bindgen` crate.
pub use wit_bindgen;

// --- INTERNAL RUNTIME & BINDINGS ---

/// Runtime support for the generated bindings.
///
/// This module is required by the `wit-bindgen` generated code.
#[doc(hidden)]
pub mod rt {
    pub use wit_bindgen::rt::*;
}

/// Generated bindings from the WIT definition.
///
/// These are hidden from the documentation to provide a cleaner user API,
/// but are public so the macro can access them.
#[doc(hidden)]
pub mod bindings {
    use super::wit_bindgen;

    wit_bindgen::generate!({
        path: "wit",
        world: "site-provider",
        runtime_path: "crate::rt",
        pub_export_macro: true,
        generate_all,
    });
}

// --- PUBLIC API ---

/// The core trait for implementing a Hydrust plugin.
///
/// Users must implement this trait to define how the plugin identifies itself
/// and how it responds to events.
///
/// **Note:** Your implementation struct must also implement [`Default`], as
/// the plugin instance is instantiated ephemerally for each call.
pub trait Handler {
    /// Returns metadata about the plugin.
    ///
    /// This is called by the host to register the plugin's name, version, and capabilities.
    fn metadata(&self) -> metadata::PluginInfo;

    /// Handles an incoming event from the host.
    ///
    /// # Arguments
    ///
    /// * `event` - The event data containing context and payload.
    fn on_event(&self, event: events::Event);
}

/// Registers a struct as the Plugin entry point.
///
/// This macro generates the necessary boilerplate to export the WASM component
/// functions expected by the Hydrust host.
///
/// # Requirements
///
/// The type passed to this macro must:
/// 1. Implement the [`Handler`] trait.
/// 2. Implement [`Default`] (or define a zero-argument `new` via `#[derive(Default)]`).
///
/// # Example
///
/// ```rust,ignore
/// register_plugin!(MyPlugin);
/// ```
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        struct GuestImpl;

        impl $crate::Guest for GuestImpl {
            fn get_info() -> $crate::metadata::PluginInfo {
                // We create a temporary instance to call the method.
                // This implies the plugin is stateless between calls in this design.
                let plugin: $plugin_type = Default::default();
                $crate::Handler::metadata(&plugin)
            }

            fn on_event(event: $crate::events::Event) {
                let plugin: $plugin_type = Default::default();
                $crate::Handler::on_event(&plugin, event);
            }
        }

        // Export the implementation using the generated macro from wit-bindgen
        $crate::bindings::export!(GuestImpl with_types_in $crate::bindings);
    };
}

// --- DOMAIN EXPORTS ---

// Re-export the Guest trait so the macro can refer to it easily,
// but users generally don't need to implement this directly.
#[doc(hidden)]
pub use bindings::Guest;

/// Functionality related to publishing content.
pub use bindings::publish;

/// Event definitions and related types.
pub mod events {
    pub use crate::bindings::hydrust::protocol::events::*;
}

/// Core data types used throughout the protocol.
pub mod types {
    pub use crate::bindings::hydrust::protocol::types::*;
}

/// Metadata structures for plugin registration.
pub mod metadata {
    pub use crate::bindings::hydrust::protocol::metadata::*;
}
