pub use plugin_macros::*;

#[derive(Debug)]
pub struct Plugin {
    pub name: String,
    pub version: String,
}

pub mod _stable {
    pub type PluginStableMainFn = extern "C" fn() -> PluginStable;
    pub const PLUGIN_INIT_FUNC_NAME: &str = "__plugin_init_func_stable";

    #[stabby::stabby]
    pub struct PluginStable {
        pub name: stabby::string::String,
        pub version: stabby::string::String,
    }

    impl From<crate::Plugin> for PluginStable {
        fn from(plugin: crate::Plugin) -> Self {
            Self {
                name: stabby::string::String::from(plugin.name),
                version: stabby::string::String::from(plugin.version),
            }
        }
    }

    impl From<PluginStable> for crate::Plugin {
        fn from(stable: PluginStable) -> Self {
            Self {
                name: stable.name.to_string(),
                version: stable.version.to_string(),
            }
        }
    }
}
