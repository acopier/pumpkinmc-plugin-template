use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::info;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn on_load(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Plugin Loaded");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Plugin unloaded. Goodbye.");
        Ok(())
    }

    fn new() -> Self {
        MyPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "MyPlugin".to_string(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: env!("CARGO_PKG_AUTHORS")
                .split(":")
                .map(|sub| sub.to_string())
                .collect(),
            description: "My plugin description.".to_string(),
            dependencies: vec![],
            permissions: vec![],
        }
    }
}

pumpkin_plugin_api::register_plugin!(MyPlugin);
