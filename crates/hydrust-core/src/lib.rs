pub mod bus;
pub mod engine;
pub mod services;

use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

// 1. Corrected bindgen macro
wasmtime::component::bindgen!({
    path: "../../wit",
    world: "site-provider",
    trappable_error_type: {},
    // Tell wasmtime to generate async bindings for these specific imports
    async: [
        "publish",
        "wasi:io/poll@0.2.0",
    ],
});

pub struct PluginHost {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for PluginHost {
    fn ctx(&mut self) -> WasiCtxView { 
        WasiCtxView::new(&mut self.ctx, &mut self.table)
    }
}

// 2. Implementation with async trait
#[async_trait::async_trait]
impl SiteProviderImports for PluginHost {
    async fn publish(&mut self, ev: hydrust::protocol::events::Event) -> wasmtime::Result<()> {
        println!("Plugin published event: {:?}", ev);
        Ok(())
    }
}

pub async fn load_plugin_metadata(wasm_bytes: &[u8]) -> wasmtime::Result<hydrust::protocol::metadata::PluginInfo> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true); 

    let engine = Engine::new(&config)?;
    let component = Component::from_binary(&engine, wasm_bytes)?;
    
    let mut linker: Linker<PluginHost> = Linker::new(&engine);

    // Use p2 (Preview 2) for the async linker
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

    // Link the generated world. 
    // This will work once the bindgen! macro above succeeds.
    SiteProvider::add_to_linker::<PluginHost, _>(&mut linker, |s| s)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build();

    let mut store = Store::new(&engine, PluginHost {
        ctx: wasi_ctx,
        table: ResourceTable::new(),
    });
    
    let (instance, _) = SiteProvider::instantiate_async(&mut store, &component, &linker).await?;
    
    // Call get_info. Bindgen with async specified makes this call async.
    let info = instance.call_get_info(&mut store).await?;
    
    Ok(info)
}