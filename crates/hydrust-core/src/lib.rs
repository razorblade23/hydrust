use wasmtime::component::bindgen;
use wasmtime::{Config, Engine, Store, component::{Component, Linker}};

// 1. Keep your bindgen macro
bindgen!({
    world: "site-provider",
    path: "../../wit/provider.wit",
    async: true,
});

// 2. Define the state that the Host will maintain
struct MyHostState;

// 3. Implement the imported functions (log, fetch-url) defined in WIT
#[async_trait::async_trait]
impl host_imports::Host for MyHostState {
    async fn fetch_url(&mut self, url: String) -> Result<Result<String, types::ErrorCode>, anyhow::Error> {
        println!("Host: Plugin requested URL: {}", url);
        // For now, return a placeholder or use reqwest here later
        Ok(Ok(format!("<html>Contents of {}</html>", url)))
    }

    async fn log(&mut self, level: String, message: String) -> Result<(), anyhow::Error> {
        println!("[PLUGIN {}]: {}", level, message);
        Ok(())
    }
}

pub async fn run_plugin(wasm_path: &str, target_url: &str) -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    
    let engine = Engine::new(&config)?;
    // Use our state here
    let mut store = Store::new(&engine, MyHostState); 
    let component = Component::from_file(&engine, wasm_path)?;

    let mut linker = Linker::new(&engine);
    
    // THIS IS THE KEY PART: Tell the linker how to provide 'host-imports'
    SiteProvider::add_to_linker(&mut linker, |state: &mut MyHostState| state)?;

    let (instance, _) = SiteProvider::instantiate_async(&mut store, &component, &linker).await?;

    // Calling the exports (can-handle, get-stream)
    let can_handle = instance.call_can_handle(&mut store, target_url).await?;
    println!("Plugin can handle URL: {}", can_handle);

    if can_handle {
        let result = instance.call_get_stream(&mut store, target_url).await?;
        match result {
            Ok(info) => println!("Found Stream! Title: {} | URL: {}", info.title, info.url),
            Err(e) => println!("Plugin error: {:?}", e),
        }
    }

    Ok(())
}