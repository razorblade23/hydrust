use hydrust_core::bus::{BusMessage, InternalCoreEvent};
use hydrust_core::engine::discovery::PluginInfo; // Import the struct!
use tokio::sync::{mpsc, broadcast};
use ratatui::DefaultTerminal;
use crate::event::{Event, EventHandler, AppEvent};

#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub struct App {
    pub running: bool,
    pub plugins: Vec<Plugin>,
    pub events: EventHandler,
    pub bus_sender: mpsc::Sender<BusMessage>,
}

impl App {
    pub fn new(bus_sender: mpsc::Sender<BusMessage>, tui_rx: broadcast::Receiver<BusMessage>) -> Self {
        Self {
            running: true,
            plugins: Vec::new(),
            events: EventHandler::new(tui_rx),
            bus_sender,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        // Trigger discovery immediately on startup
        let _ = self.bus_sender.send(BusMessage::Internal(InternalCoreEvent::PluginDiscovery)).await;

        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            
            match self.events.next().await? {
                Event::Tick => {},
                Event::Crossterm(e) => self.handle_key_events(e)?,
                
                // NEW: Handle the Bus Message
                Event::Bus(msg) => self.handle_bus_message(msg),
                
                Event::App(e) => { /* handle app events */ }
            }
        }
        Ok(())
    }

    fn handle_bus_message(&mut self, msg: BusMessage) {
        if let BusMessage::Internal(InternalCoreEvent::PluginDiscoveryComplete(discovered)) = msg {
            // Map the Core PluginInfo to TUI Plugin struct
            self.plugins = discovered.into_iter().map(|p| Plugin {
                name: p.name,
                version: p.version,
                description: p.description.unwrap_or_default(),
            }).collect();
        }
    }
}