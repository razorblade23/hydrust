use hydrust_core::bus::{BusMessage, InternalCoreEvent};
use tokio::sync::{mpsc, broadcast};

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    pub plugins: Vec<Plugin>,
    /// Event handler.
    pub events: EventHandler,
    pub bus_sender: mpsc::Sender<BusMessage>,
}

#[derive(Debug)]
pub struct PluginList {
    items: Vec<Plugin>,
    state: ListState,
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             running: true,
//             counter: 0,
//             plugins: Vec::new(),
//             events: EventHandler::new(),
//         }
//     }
// }

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(bus_sender: mpsc::Sender<BusMessage>, tui_rx: broadcast::Receiver<BusMessage>) -> Self {
        Self {
            running: true,
            counter: 0,
            plugins: Vec::new(),
            // Pass the bus receiver into our event handler
            events: EventHandler::new(tui_rx),
            bus_sender,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => { /* handle keys */ },
                
                // NEW: Handle messages from the actual Engine/Plugins
                Event::Bus(msg) => self.handle_bus_message(msg),

                Event::App(app_event) => match app_event {
                    AppEvent::DiscoverPlugins => self.request_plugin_discovery(),
                    AppEvent::Quit => self.quit(),
                    _ => {}
                },
            }
        }
        Ok(())
    }

    /// Instead of calling discovery directly (blocking), we ask the Engine to do it
    pub fn request_plugin_discovery(&mut self) {
        let tx = self.bus_sender.clone();
        tokio::spawn(async move {
            let _ = tx.send(BusMessage::Internal(InternalCoreEvent::PluginDiscovery)).await;
        });
    }

    /// React to what the Engine found
    fn handle_bus_message(&mut self, msg: BusMessage) {
    match msg {
        // We receive the full Vec of PluginInfo from the Core
        BusMessage::Internal(InternalCoreEvent::PluginDiscoveryComplete(discovered_plugins)) => {
            self.plugins = discovered_plugins
                .into_iter()
                .map(|p| Plugin {
                    id: p.id,
                    name: p.name,
                    version: p.version,
                    author: p.author,
                    description: p.description,
                })
                .collect();
            
            // You can now log something more descriptive!
            // println!("Loaded {} plugins from disk.", self.plugins.len());
        }

        BusMessage::External(event) => {
            // This is where events from the WASM guest (via publish) land.
            // e.g., if a plugin sends a 'Log' event or 'DiscoveryResult'
        }

        _ => {}
    }
}

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char('s' | 'S') => self.events.send(AppEvent::DiscoverPlugins),
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn send_app_event(&mut self, event: AppEvent) {
        self.events.send(event);
    }
}
