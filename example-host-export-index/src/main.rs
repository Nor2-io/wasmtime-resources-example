use anyhow::anyhow;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

//TODO: Change resources from hashmap to allow for more than one implementation of the same resource type
bindgen!({
    path: "./example2.wit",
    resources: {
        "scalars": ImplScalars
    }
});

pub trait ResourceDrop<T> {
    fn drop(store: &mut StoreContextMut<'_, T>, rep: u32) -> wasmtime::Result<()>;
}

#[derive(Debug, Default)]
pub struct Host;

impl crate::example2::component::backend::Host for Host {
    fn fetch(&mut self, _url: String) -> wasmtime::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Default)]
pub struct State {
    host: Host,
    scalars: Option<ImplScalars>,
}

#[derive(Debug, Default)]
pub struct ImplScalars;

impl<T> ResourceDrop<T> for ImplScalars {
    fn drop(store: &mut wasmtime::StoreContextMut<'_, T>, rep: u32) -> wasmtime::Result<()> {
        //TODO: Don't we need the the generic in Store<T> to implement a trait to be able to access the data and drop the handle here?

        Ok(())
    }
}

impl crate::example2::component::backend::Scalars for ImplScalars {
    fn new<T>(
        _store: &mut wasmtime::StoreContextMut<'_, T>,
    ) -> wasmtime::Result<(Self, wasmtime::component::Resource<Self>)>
    where
        Self: Sized,
    {
        Ok((Self, wasmtime::component::Resource::<Self>::new_own(1)))
    }
    fn method_scalars_get_b(
        &self,
        _self_: wasmtime::component::Resource<Self>,
    ) -> wasmtime::Result<u32>
    where
        Self: Sized,
    {
        Ok(5)
    }
}

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "./example_guest_import.wasm").unwrap();
    let mut linker = Linker::new(&engine);

    let mut store = Store::new(&engine, State::default());

    crate::example2::component::backend::add_to_linker::<State, ImplScalars, Host>(
        &mut linker,
        |s| &mut s.host,
        |s, rep| {
            //In a real implementetion you would lookup the resource based on the argument `rep`
            let resource = std::mem::take(&mut s.scalars);
            resource.unwrap()
        },
    )?;

    let (bindings, _instance) = Example2::instantiate(&mut store, &component, &linker).unwrap();

    let test = bindings
        .example2_component_front()
        .call_handle(
            &mut store,
            wasmtime::component::Resource::<ImplScalars>::new_own(1),
        )
        .unwrap();
    dbg!(test);

    Ok(())
}
