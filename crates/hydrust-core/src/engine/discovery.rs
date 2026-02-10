

pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

impl PluginInfo {
    pub fn new(id: String, name: String, version: String, author: String, description: String) -> Self {
        Self {
            id,
            name,
            version,
            author,
            description,
        }
    }
}

pub fn discover_plugins() -> Vec<PluginInfo> {
    // Placeholder for plugin discovery logic
    // In a real implementation, this would scan directories, read metadata files, etc.
    vec![
        PluginInfo::new(
            "plugin-1".to_string(),
            "Example Plugin".to_string(),
            "0.1.0".to_string(),
            "Author Name".to_string(),
            "A sample plugin for demonstration purposes.".to_string(),
        ),
        PluginInfo::new(
            "plugin-2".to_string(),
            "Another Plugin".to_string(),
            "0.2.0".to_string(),
            "Another Author".to_string(),
            "Another sample plugin for testing.".to_string(),
        ),
    ]
}