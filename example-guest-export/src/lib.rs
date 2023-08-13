cargo_component_bindings::generate!();

use bindings::exports::example::component::backend::{Backend, Scalars as ScalarsTrait};
pub struct Scalars;

impl ScalarsTrait for Scalars {
    fn new() -> Self {
        Self
    }
    fn get_a() -> u32 {
        5
    }
    fn get_b(&self) -> u32 {
        2
    }
}

struct Component;

impl Backend for Component {
    fn fetch(_url: String) -> Vec<u8> {
        Vec::new()
    }

    fn scalar_arg(x: &Scalars) -> u32 {
        let t = x.get_b();
        t + 5
    }
}
