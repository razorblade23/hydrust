pub mod bus;
pub mod engine;
pub mod host;
pub mod services;


use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView, ResourceTable};
use crate::bus::BusMessage;
use tokio::sync::mpsc;

// Generate bindings
wasmtime::component::bindgen!({
    path: "../../wit",
    world: "site-provider",
    trappable_error_type: {},
    async: true, // Required for async_support(true) in Wasmtime 41
});

/// The State held by the WASM Host
pub struct PluginHost {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for PluginHost {
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
}

// In Wasmtime 41, async worlds generate async traits
#[async_trait::async_trait]
impl SiteProviderImports for PluginHost {
    async fn publish(&mut self, ev: hydrust::protocol::events::Event) -> wasmtime::Result<()> {
        println!("Plugin published event: {:?}", ev);
        Ok(())
    }
}

// These are simple data traits, no async needed here
impl hydrust::protocol::types::Host for PluginHost {}
impl hydrust::protocol::events::Host for PluginHost {}
impl hydrust::protocol::metadata::Host for PluginHost {}

pub struct PluginInstance {
    pub plugin: engine::discovery::PluginInfo,
}

pub async fn load_plugin_metadata(wasm_bytes: &[u8]) -> wasmtime::Result<hydrust::protocol::metadata::PluginInfo> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true); 

    let engine = Engine::new(&config)?;
    let component = Component::from_binary(&engine, wasm_bytes)?;
    
    // Explicitly type the linker to help the compiler resolve the 'state' closure
    let mut linker: Linker<PluginHost> = Linker::new(&engine);

    // 1. Link WASI (Preview 2 is standard in 41.x)
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

    // 2. Link your custom world. 
    // We remove the explicit type on 'state' to avoid the HasData conflict
    SiteProvider::add_to_linker(&mut linker, |s| s)?;

    // 3. Setup the context
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build();

    let mut store = Store::new(&engine, PluginHost {
        ctx: wasi_ctx,
        table: ResourceTable::new(),
    });
    
    // 4. Instantiate and Call (Note the .await on BOTH calls)
    let (instance, _) = SiteProvider::instantiate_async(&mut store, &component, &linker).await?;
    
    // Call get-info() - this is now an async call because of bindgen! settings
    let info = instance.call_get_info(&mut store).await?;
    
    Ok(info)
}