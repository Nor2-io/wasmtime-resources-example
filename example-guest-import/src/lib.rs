cargo_component_bindings::generate!();
use bindings::exports::example2::component::front::{Front, Scalars};

struct Component;

impl Front for Component {
    fn handle(x: Scalars) -> u32 {
        x.get_b()
    }
}
