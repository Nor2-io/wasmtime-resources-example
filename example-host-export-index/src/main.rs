use anyhow::anyhow;
use wasmtime::{component::*, AsContextMut};
use wasmtime::{Config, Engine, Store};

use crate::example2::component::backend::Scalars;

bindgen!({
    path: "./example2.wit"
});

#[derive(Debug, Default)]
pub struct Host;

impl example2::component::backend::Host for Host {
    fn fetch(&mut self, _url: String) -> wasmtime::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Default)]
pub struct State {
    host: Host,
    scalars_table: std::collections::HashMap<u32, ImplScalars>,
}

#[derive(Debug)]
pub struct ImplScalars;

impl example2::component::backend::Scalars for State {
    type Resource = ImplScalars;

    fn new(
        store: &mut wasmtime::StoreContextMut<'_, Self>,
    ) -> wasmtime::Result<wasmtime::component::Resource<Self::Resource>>
    where
        Self: Sized,
    {
        let state = store.data_mut();

        let handle =
            wasmtime::component::Resource::<ImplScalars>::new_own(state.scalars_table.len() as u32);
        state.scalars_table.insert(handle.rep(), ImplScalars);

        Ok(handle)
    }

    fn method_scalars_get_b(
        &mut self,
        _self_: wasmtime::component::Resource<Self::Resource>,
    ) -> wasmtime::Result<u32>
    where
        Self: Sized,
    {
        Ok(5)
    }

    fn drop(mut store: wasmtime::StoreContextMut<'_, Self>, rep: u32) -> wasmtime::Result<()> {
        let state = store.data_mut();

        state.scalars_table.remove(&rep).ok_or(anyhow!(
            "tried to drop a resource `{}` that doesn't exist",
            rep
        ))?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "./example_guest_import.wasm").unwrap();
    let mut linker = Linker::new(&engine);

    let state = State::default();

    let mut store = Store::new(&engine, state);
    example2::component::backend::add_to_linker::<State, Host>(&mut linker, |s| &mut s.host)?;

    let (bindings, _instance) = Example2::instantiate(&mut store, &component, &linker).unwrap();

    let resource = State::new(&mut store.as_context_mut())?;

    let test = bindings
        .example2_component_front()
        .call_handle(&mut store, resource)
        .unwrap();

    dbg!(test);

    Ok(())
}
