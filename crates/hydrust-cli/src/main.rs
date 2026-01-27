use clap::Parser;
use std::path::PathBuf;

/// Hydrust CLI - High-performance media archiving
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .wasm plugin file
    #[arg(short, long)]
    plugin: PathBuf,

    /// The URL of the video/stream to archive
    #[arg(index = 1)]
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("🌊 Hydrust Engine Initializing...");
    println!("Loading plugin: {:?}", args.plugin);

    // Ensure the plugin file exists before calling the core
    if !args.plugin.exists() {
        anyhow::bail!("Plugin file not found at {:?}", args.plugin);
    }

    // Call the core loader we just built
    match hydrust_core::run_plugin(
        args.plugin.to_str().unwrap(), 
        &args.url
    ).await {
        Ok(_) => println!("\n✅ Task completed successfully."),
        Err(e) => eprintln!("\n❌ Engine Error: {}", e),
    }

    Ok(())
}