pub use wit_bindgen;

// The generated code often looks for 'wit_bindgen::rt'
// We make sure 'hydrust_sdk::rt' exists and points to the right place.
pub mod rt {
    pub use wit_bindgen::rt::*;
}

#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ident) => {
        // THE FIX: 'extern crate' allows us to alias a dependency crate 
        // as a different name in the root of the current crate.
        // This makes '::wit_bindgen' point to 'hydrust_sdk' globally in this plugin.
        extern crate hydrust_sdk as wit_bindgen;

        // We call generate! through our alias
        wit_bindgen::wit_bindgen::generate!({
            inline: "
                package hydrust:protocol;
                interface types {
                    record stream-info { title: string, url: string }
                    enum error-code { network-error, invalid-url, other }
                }
                world site-provider {
                    use types.{stream-info, error-code};
                    export can-handle: func(url: string) -> bool;
                    export get-stream: func(url: string) -> result<stream-info, error-code>;
                }
            ",
            world: "site-provider",
            // We tell it the runtime is available via 'wit_bindgen'
            // (which now points to 'hydrust_sdk' via our extern crate alias)
            runtime_path: "wit_bindgen::rt",
        });

        export!($plugin_type);
    };
}