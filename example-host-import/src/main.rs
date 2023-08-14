use exports::example::component::backend::{RepScalars, Scalars};
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!({
    path: "./example.wit",
});

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "./example_guest_export.wasm").unwrap();
    let linker = Linker::new(&engine);

    let mut store = Store::new(&engine, ());

    let (bindings, instance) = Example::instantiate(&mut store, &component, &linker).unwrap();

    let resource = RepScalars::new(&mut store, &instance).unwrap();

    //TODO: Fix so you can pass any resource representation implementing the trait and `ToHandle`
    // Instead of the below it will allow you to pass `resource` directly as an argument:
    //  let test = bindings
    //    .interface0
    //    .call_scalar_arg(&mut store, resource)
    //    .unwrap();

    let test = bindings
        .interface0
        .call_scalar_arg(&mut store, resource.handle)
        .unwrap();

    dbg!(test);

    Ok(())
}
