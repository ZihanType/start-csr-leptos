use {{crate_name}}::App;
use leptos::prelude::*;
use tracing::Level;
use tracing_wasm::WASMLayerConfigBuilder;

fn main() {
    let config = WASMLayerConfigBuilder::new()
        .set_max_level(Level::DEBUG)
        .build();

    tracing_wasm::set_as_global_default_with_config(config);

    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
