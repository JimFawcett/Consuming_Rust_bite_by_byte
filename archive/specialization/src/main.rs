// specialization::main.rs
// - nightly only
// - incomplete

use std::default::Default;

#[derive(Debug)]
struct DemoSpec<T> where T:Default {
    t:T
}
impl<T>  DemoSpec<T> where T:Default {
    fn new() ->DemoSpec<T> {
        DemoSpec {
            t: Default::default(),
        }
    }
    fn show(&self) {
        print!("\n  DemoSpec::show - for generic types");
        print!("\n  {:?}", std::any::type_name::<T>());
    }
}
impl<int> DemoSpec<int> {
    fn show<int>(&self) {
        print!("\n  DemoSpec::show - for generic types");
        print!("\n  {:?}", std::any::type_name::<T>());
    }
}
fn main() {
    let ds = DemoSpec::<i32>::new();
    println!("Hello, world!");
}
