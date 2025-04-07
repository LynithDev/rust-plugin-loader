use proc_macro2::Span;
use syn::{parse_macro_input, Ident, ItemFn, Visibility};

#[proc_macro_attribute]
pub fn plugin(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    func.vis = Visibility::Inherited;
    func.sig.ident = Ident::new("__plugin_init_func", Span::call_site());

    let output = quote::quote! {
        #func

        #[stabby::export]
        pub extern "C" fn __plugin_init_func_stable() -> plugin_api::_stable::PluginStable {
            __plugin_init_func().into()
        }
    };

    output.into()
}