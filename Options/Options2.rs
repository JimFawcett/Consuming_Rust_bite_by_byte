/*-------------------------------------
  Pluggin Architecture using Option<T>
  - Version 2 
  - holds Option of T
  - does't require annotating lifetimes
    since Pluggin is moved into Demo,
    e.g., Demo now owns Pluggin
  - also shows how to access pluggin
    from caller
*/
use std::fmt::Debug;

/*------------------------------------
  Trait declared by Demo and 
  implemented by Pluggin
*/
pub trait Plug {
    fn do_plug_op(&self, msg: &str);
    fn name(&mut self, name: &str);
}
/*-------------------------------------
  Demo accepts a Pluggin that supplies
  a service for Demo to use.
*/
#[derive(Debug)]
pub struct Demo<T> where T: Plug + Debug + Clone {
    pluggin: Option<T>,
}
impl<T> Demo<T> where T: Plug + Debug + Clone {
    pub fn new() -> Self {
        Self {
            pluggin: None,
        }
    }
    pub fn do_op(&mut self, msg: &str) {
        print!("\n  {}", msg);
        if let Some(plug) = &mut self.pluggin {
            plug.name("Jeff");
            plug.do_plug_op("pluggin called by Demo");
        } 
    }
    /*-- register pluggin --*/
    pub fn accept(&mut self, p: T) {
        self.pluggin = Some(p);
    }
    /*-- get clone of pluggin if it exists --*/
    pub fn pluggin(&self) -> Option<T> {
        self.pluggin.clone()
    }
}
/*-----------------------------------
  Pluggin is a service provider that
  implements Plug trait so Demo
  knows how to call it.
*/
#[derive(Debug, Clone)]
pub struct Pluggin {
    name: String
}
impl Plug for Pluggin {
    fn do_plug_op(&self, msg: &str) {
        print!("\n  {}: {}", &self.name, msg);
    }
    fn name(&mut self, n:&str) {
        self.name = n.to_string();
    }
}
impl Pluggin {
    fn new() -> Self {
        Self { name: "".to_string() }
    }
}
/*-------------------------------------
  Here's Pluggin architecture at work
*/
fn main() {
    let mut p = Pluggin::new();
    p.name("Fred");
    p.do_plug_op("pluggin call from main");
    let mut d = Demo::<Pluggin>::new();
    d.accept(p);
    /*-- use Demo with pluggin --*/
    d.do_op("demo<Pluggin>::do_op called");
    /*-- retrieve pluggin --*/
    let opt = d.pluggin();
    if let Some(pl) = opt {
        pl.do_plug_op("pluggin called from main");
    }
    print!("\n\n  That's all Folks!\n\n");
}