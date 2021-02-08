/*-------------------------------------
  Pluggin Architecture using Option<T>
  - Version 1 
  - holds Option of reference to T
  - requires annotating lifetimes
    so Rust can ensure that Demo
    doesn't outlive its pluggin
*/
use std::fmt::Debug;

/*------------------------------------
  Trait declared by Demo and 
  implemented by Pluggin
*/
pub trait Plug {
    fn do_plug_op(&self, msg: &str);
    fn name(&mut self, n: &str);
}
/*-------------------------------------
  Demo accepts a Pluggin that supplies
  a service for Demo to use.
*/
#[derive(Debug)]
pub struct Demo<'a, T> where T: Plug + Debug + 'a {
    pluggin: Option<&'a mut T>,
}
impl<'a, T> Demo<'a, T> where T: Plug + Debug + 'a {
    pub fn new() -> Self {
        Self {
            pluggin: None,
        }
    }
    pub fn do_op(&mut self, msg: &str) {
        print!("\n  {}", msg);
        /*-- show Demo can change state of pluggin --*/
        if self.pluggin.is_some() {
            /*-- get inner &T as mutable --*/
            let pl = self.pluggin.as_mut().unwrap();
            pl.name("Joe");
            pl.do_plug_op("pluggin called from Demo");
        } 
    }
    pub fn accept(&mut self, p: &'a mut T) {
        self.pluggin = Some(p);
    }
}
/*-----------------------------------
  Pluggin is a service provider that
  implements Plug trait so Demo
  knows how to call it.
*/
#[derive(Debug)]
pub struct Pluggin {
    name: String
}
impl Plug for Pluggin {
    fn do_plug_op(&self, msg: &str) {
        print!("\n  {}: {}", &self.name, msg);
    }
    fn name(&mut self, n: &str) {
        self.name = n.to_string();
    }
}
impl Pluggin {
    pub fn new() -> Self {
        Self { name: "".to_string() }
    }
}
/*-------------------------------------
  Here's Pluggin architecture at work
  - show that caller can access state
    of pluggin after Demo changes it
*/
fn main() {
    let mut p = Pluggin::new();
    p.name("Fred");
    p.do_plug_op("pluggin called from main");
    
    let mut d = Demo::<Pluggin>::new();
    d.accept(&mut p);
    /*-- use Demo with pluggin --*/
    d.do_op("demo<Pluggin>::do_op called");
    
    /*-- use original pluggin --*/
    p.do_plug_op("original pluggin called from main");

    print!("\n\n  That's all Folks!\n\n");
}