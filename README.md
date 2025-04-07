# Rust Plugin Loader
Uses libloading and stabby

# Crates
- [`crates/api`](./crates/api/) - Definitions (as well as FFI-stable ones) for the API
- [`crates/loader`](./crates/loader/) - The loader in the form of an executable
- [`crates/macros`](./crates/macros/) - Macros re-exported by the API crate, makes writing a plugin easier
- [`crates/plugin`](./crates/plugin/) - The plugin in the form of a library

## Example Plugin
```rust
use plugin_api::Plugin;

#[plugin_api::plugin]
pub fn main() -> Plugin {
    Plugin {
        name: "Test Plugin".to_string(),
        version: "0.1.0".to_string(),
    }
}
```