// core/src/lib.rs
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

// Generate bindings with trappable errors
wasmtime::component::bindgen!({
    path: "../../wit",
    world: "site-provider",
    trappable_error_type: {},
});

pub struct PluginHost;

impl wasmtime::component::HasData for PluginHost {
    type Data<'a> = &'a mut PluginHost;
}

// Implement the imports - now they return Result
impl SiteProviderImports for PluginHost {
    fn publish(&mut self, ev: hydrust::protocol::events::Event) -> () {
        println!("Plugin published event: {:?}", ev);
        ()
    }
}

impl hydrust::protocol::types::Host for PluginHost {}
impl hydrust::protocol::events::Host for PluginHost {}

pub fn run_plugin(wasm_bytes: &[u8]) -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    
    let engine: Engine = Engine::new(&config)?;
    let component: Component = Component::from_binary(&engine, wasm_bytes)?;
    
    let mut linker: Linker<PluginHost> = Linker::new(&engine);
    SiteProvider::add_to_linker::<PluginHost, PluginHost>(
        &mut linker, 
        |state| state
    )?;
    
    let mut store = Store::new(&engine, PluginHost);
    let instance = SiteProvider::instantiate(&mut store, &component, &linker)?;
    
    let test_event = hydrust::protocol::events::Event {
        id: "test-123".to_string(),
        origin: "core".to_string(),
        timestamp: 0,
        payload: hydrust::protocol::events::EventPayload::Core(
            hydrust::protocol::events::CoreEvent::IntentResolve(
                "https://youtube.com/watch?v=test".to_string()
            )
        ),
    };
    
    instance.call_on_event(&mut store, &test_event)?;
    
    Ok(())
}