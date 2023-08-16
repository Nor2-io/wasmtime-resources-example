use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

//TODO: Change resources from hashmap to allow for more than one implementation of the same resource type
bindgen!({
    path: "./example2.wit",
    resources: {
        "scalars": ImplScalars
    }
});

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
    //TODO: Add proc macro to automatically implement ResourceTable and add table to store state
    scalars_table: std::collections::HashMap<u32, ImplScalars>,
}

#[derive(Debug, Default)]
pub struct ImplScalars; // Fwiw in wasi i have been calling these structs HostScalars

// I think we need this trait to also give us access to `State` some way or another (or, really, the `Host` element in state)
// Maybe it needs an associated type `type Context = Host`?
impl crate::example2::component::backend::Scalars for ImplScalars { 
    fn new() -> wasmtime::Result<Self>
    // In particular I would expect to need a `&mut Host` in order to write a useful constructor here.
    where
        Self: Sized,
    {
        Ok(ImplScalars)
    }

    fn method_scalars_get_b(&self) -> wasmtime::Result<u32> // I would expect this to take &mut self
    where
        Self: Sized,
    {
        Ok(5)
    }
}

impl wasmtime::component::ResourceTable<ImplScalars> for State {
    // just curious why there is both a shared and mut getter. Where does the mut getter end up being used? the above method takes &self
    fn get_resource(&self, handle: wasmtime::component::Resource<ImplScalars>) -> &ImplScalars {
        self.scalars_table.get(&handle.rep()).unwrap()
    }

    fn new_resource(
        &mut self,
        resource: ImplScalars,
    ) -> wasmtime::component::Resource<ImplScalars> {
        let handle =
            wasmtime::component::Resource::<ImplScalars>::new_own(self.scalars_table.len() as u32);
        self.scalars_table.insert(handle.rep(), resource);

        handle
    }

    fn get_resource_mut(
        &mut self,
        handle: wasmtime::component::Resource<ImplScalars>,
    ) -> &mut ImplScalars {
        self.scalars_table.get_mut(&handle.rep()).unwrap()
    }

    fn drop_resource(&mut self, rep: u32) {
        self.scalars_table
            .remove(&rep)
            .unwrap_or_else(|| panic!("tried to drop a resource that doesn't exist"));
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

    crate::example2::component::backend::add_to_linker::<State, Host>(&mut linker, |s| {
        &mut s.host
    })?;

    let (bindings, _instance) = Example2::instantiate(&mut store, &component, &linker).unwrap();

    let test = bindings
        .example2_component_front()
        .call_handle(&mut store, ImplScalars)
        .unwrap();
    dbg!(test);

    Ok(())
}
