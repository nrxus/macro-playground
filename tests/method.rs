#![allow(dead_code)]

use macro_playground::method;

struct SomeStruct;

impl SomeStruct {
    #[method]
    fn some_method(self: std::rc::Rc<Self>) {}
}
