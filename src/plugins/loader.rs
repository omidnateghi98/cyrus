// src/plugins/loader.rs
use anyhow::Result;
use std::path::Path;
use libloading::Library;
use crate::plugins::{LoadedPlugin, PluginManifest};
use crate::plugins::interface::CyrusPlugin;

pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        Self
    }

    pub async fn load_plugin(
        &self,
        library_path: &Path,
        manifest: PluginManifest,
    ) -> Result<LoadedPlugin> {
        // Load the dynamic library
        let library = unsafe { Library::new(library_path)? };

        // Get the plugin creation function
        let create_plugin: libloading::Symbol<unsafe extern "C" fn() -> *mut std::os::raw::c_void> =
            unsafe { library.get(b"cyrus_plugin_create")? };

        let plugin_ptr = unsafe { create_plugin() };
        let plugin: Box<dyn CyrusPlugin + Send + Sync> = unsafe {
            *Box::from_raw(plugin_ptr as *mut Box<dyn CyrusPlugin + Send + Sync>)
        };

        let info = plugin.get_info();

        Ok(LoadedPlugin {
            info,
            manifest,
            library,
            plugin,
            enabled: false,
        })
    }
}