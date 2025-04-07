use plugin_api::Plugin;
use stabby::libloading::StabbyLibrary;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];

    let plugin = unsafe {
        let lib = libloading::Library::new(path).unwrap_or_else(|_| {
            eprintln!("Failed to load library: {}", path);
            std::process::exit(1);
        });

        let symbol = lib.get_stabbied::<plugin_api::_stable::PluginStableMainFn>(b"__plugin_init_func_stable")
            .expect("Failed to find init_plugin function");

        Plugin::from(symbol())
    };

    
    println!("{:#?}", plugin);
}
