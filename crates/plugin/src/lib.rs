use plugin_api::Plugin;

#[plugin_api::plugin]
pub fn main() -> Plugin {
    Plugin {
        name: "Test Plugin".to_string(),
        version: "0.1.0".to_string(),
    }
}