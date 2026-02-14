use crate::plugins::traits::Plugin;
use crate::core::errors::EngineResult;
use std::collections::HashMap;

/// ============================================================================
/// 📚 Plugin Registry (ප්ලගින ලේඛනය)
/// ============================================================================
/// සියලුම ප්ලගින කළමනාකරණය කරයි.

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> EngineResult<()> {
        plugin.on_load()?;
        self.plugins.insert(plugin.name().to_string(), plugin);
        Ok(())
    }

    pub fn unregister(&mut self, name: &str) -> EngineResult<()> {
        if let Some(plugin) = self.plugins.remove(name) {
            plugin.on_unload()?;
        }
        Ok(())
    }
}
