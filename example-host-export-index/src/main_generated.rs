extern crate std;
use wasmtime::{component::*, StoreContextMut};
use wasmtime::{Config, Engine, Store};
pub struct Example2 {
    interface0: exports::example2::component::front::Front,
}

pub trait ResourceDrop<T> {
    fn drop(store: &mut StoreContextMut<'_, T>, rep: u32) -> wasmtime::Result<()>;
}

const _: () = {
    impl Example2 {
        pub fn add_to_linker<T, R, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            get_resource: impl Fn(&mut T, u32) -> R + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            U: example2::component::backend::Host,
            R: crate::example2::component::backend::Scalars + crate::ResourceDrop<T> + 'static,
        {
            example2::component::backend::add_to_linker(linker, get, get_resource)?;
            Ok(())
        }
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate(&mut store)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let interface0 = exports::example2::component::front::Front::new(
                &mut __exports
                    .instance("example2:component/front@0.1.0")
                    .ok_or_else(|| {
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(format_args!(
                                "exported instance `example2:component/front@0.1.0` not present"
                            ));
                            error
                        })
                    })?,
            )?;
            Ok(Example2 { interface0 })
        }
        pub fn example2_component_front(&self) -> &exports::example2::component::front::Front {
            &self.interface0
        }
    }
};
pub mod example2 {
    pub mod component {
        #[allow(clippy::all)]
        pub mod backend {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub trait Scalars {
                fn new<T>(
                    store: &mut wasmtime::StoreContextMut<'_, T>,
                ) -> wasmtime::Result<(Self, wasmtime::component::Resource<Self>)>
                where
                    Self: Sized;
                fn method_scalars_get_b(
                    &self,
                    self_: wasmtime::component::Resource<Self>,
                ) -> wasmtime::Result<u32>
                where
                    Self: Sized;
            }

            pub trait Host {
                fn fetch(&mut self, url: String) -> wasmtime::Result<Vec<u8>>;
            }
            pub fn add_to_linker<T, R, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                get_resource: impl Fn(&mut T, u32) -> R + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                R: Scalars + crate::ResourceDrop<T> + 'static,
                U: Host,
            {
                let mut inst = linker.instance("example2:component/backend@0.1.0")?;
                inst.resource::<R>("scalars", |mut store, rep| {
                    //TODO: Shouldn't this trap so we could consume fuel among other things in the drop call?
                    R::drop(&mut store, rep).unwrap();
                })?;
                inst.func_wrap(
                    "[constructor]scalars",
                    move |mut caller: wasmtime::StoreContextMut<'_, _>, (): ()| {
                        let (resource, handle) = R::new(&mut caller)?;
                        //TODO: Don't we need the the generic in Store<T> to implement a trait to store the handle here?

                        Ok((handle,))
                    },
                )?;
                inst.func_wrap(
                    "[method]scalars.get-b",
                    move |mut caller: wasmtime::StoreContextMut<'_, _>,
                          (arg0,): (wasmtime::component::Resource<R>,)| {
                        let r = get_resource(caller.data_mut(), arg0.rep());
                        let r = r.method_scalars_get_b(arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "fetch",
                    move |mut caller: wasmtime::StoreContextMut<'_, _>, (arg0,): (String,)| {
                        let host = get(caller.data_mut());
                        let r = host.fetch(arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
}
pub mod exports {
    pub mod example2 {
        pub mod component {
            #[allow(clippy::all)]
            pub mod front {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub struct Front {
                    handle: wasmtime::component::Func,
                }
                impl Front {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Front> {
                        let handle = *__exports
                            .typed_func::<(wasmtime::component::ResourceAny,), (u32,)>("handle")?
                            .func();
                        Ok(Front { handle })
                    }
                    pub fn call_handle<R0: 'static, S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::Resource<R0>,
                    ) -> wasmtime::Result<u32> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::Resource<R0>,),
                                (u32,),
                            >::new_unchecked(self.handle)
                        };
                        let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}

const _: &str = "package example2:component@0.1.0\r\n\r\ninterface backend {\r\n    resource scalars {\r\n        constructor()\r\n        get-b: func() -> u32\r\n\r\n    }\r\n\r\n    fetch: func(url: string) -> list<u8>\r\n}\r\n\r\ninterface front {\r\n  
  use backend.{scalars}\r\n\r\n    handle: func(x: own<scalars>) -> u32\r\n}\r\n\r\nworld example2 {\r\n    import backend\r\n  
  export front\r\n}";

// USER IMPLEMENTED FROM HERE

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
    fn drop(store: &mut StoreContextMut<'_, T>, rep: u32) -> wasmtime::Result<()> {
        //TODO: Don't we need the the generic in Store<T> to implement a trait to be able to access the data and drop the handle here?

        Ok(())
    }
}

impl crate::example2::component::backend::Scalars for ImplScalars {
    fn new<T>(
        _store: &mut StoreContextMut<'_, T>,
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

fn main() -> wasmtime::Result<()> {
    let body = async {
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
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
