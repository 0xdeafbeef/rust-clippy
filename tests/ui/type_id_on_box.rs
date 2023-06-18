//@run-rustfix

#![warn(clippy::type_id_on_box)]

use std::any::{Any, TypeId};

fn existential() -> impl Any {
    Box::new(1) as Box<dyn Any>
}

fn main() {
    let any_box: Box<dyn Any> = Box::new(0usize);
    let _ = any_box.type_id();
    let _ = TypeId::of::<Box<dyn Any>>(); // don't lint, user probably explicitly wants to do this
    let _ = (*any_box).type_id();
    let any_box: &Box<dyn Any> = &(Box::new(0usize) as Box<dyn Any>);
    let _ = any_box.type_id(); // 2 derefs are needed here
    let b = existential();
    let _ = b.type_id(); // don't lint
}
