/////////////////////////////////////////////////////////////
// methods_with_threads::main.rs - member thread helper    //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 23 Jul 2020  //
/////////////////////////////////////////////////////////////
/*
   Making methods asynchronous often leads to compile errors
   with messages about "can't infer appropriate lifetime ..."

   These examples show why that occurs and how to fix the
   problem
*/

#![allow(dead_code)]

use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};

struct Test1 {
    count: Arc<Mutex<i32>>,
}
impl Test1 {
    fn new() -> Test1 {
        Test1 {
            count: Arc::new(Mutex::new(0)),
        }
    }
    /*-----------------------------------------------------
      Uncommenting the loop, below, causes compile error.
      The problem is that Rust can't guarantee that the
      reference to count will remain valid.  The thread 
      lifetime could exceed the lifetime of Test1 instance.
    */
    fn start(&mut self) -> JoinHandle<()> {
        std::thread::spawn(move || {
            // for i in 0..5 {
            //     let mut data = self.count.lock().unwrap();
            //     *data += i;
            //     print!("\n  {:?}", data);
            // }
        })
    }
}

struct Test1a {
    count: Arc<Mutex<i32>>,
}
impl Test1a {
    fn new() -> Test1a {
        Test1a {
            count: Arc::new(Mutex::new(0)),
        }
    }
    /*-----------------------------------------------------
      Instead of using member data in Test1a::start we've
      used a temporary scount, then shared its value with
      self.count at the end.
    */
    fn get_initial_value(&self) -> i32 {
        *self.count.lock().unwrap()
        /* return unlocks mutex */
    }
    fn start(&mut self) -> JoinHandle<()> {
        /* scount is pointer to a heap value that can be shared */
        let iv = self.get_initial_value();
        let scount = Arc::new(Mutex::new(iv));  // moved into thread
        /* share refers to the same value as scount */
        let share = Arc::clone(&scount);
        let handle = std::thread::spawn(move || {  /* scount moved */
            for i in 0..5 {
                let mut data = scount.lock().unwrap();
                *data += i;
                print!("\n  {:?}", data);
                /* data unlocked here*/
            }
        });
        /*-------------------------------------------------
          scount is invalid in this scope (been moved into thread), 
          but share is valid ref to value also referenced by scount,
          since an Arc's value is not dropped until all shared
          references are dropped.
        */
        self.count = share;
        handle
    }
    fn show_count(&self) {
        print!("\n\n  t1a result = {:?}",self.count.lock().unwrap());
    }
}

struct Test2 {
    counter: Arc<Mutex<i32>>,
}
impl Test2 {
    /*-----------------------------------------------------
      This example uses the same technique as Test1a, but
      moves the thread into new(), the Test2 constructor.  
      That would be a good idea for any type whose instances 
      need the running thread to operate as expected. 

      Note that new() now returns a tuple with the newly
      constructed Test2 instance and the thread handle.
      Look at main to see how that is used.
    */
    fn new() -> (Test2, JoinHandle<()>) {
        let scount = Arc::new(Mutex::new(0));
        let share = Arc::clone(&scount);
        let handle = std::thread::spawn(move || {  // scount moved
            for i in 0..5 {
                let mut data = scount.lock().unwrap();
                *data += i;
                print!("\n  {:?}", data);
            }
        });
        (
            Test2 { counter: share, },  
            handle
        )
    }
    fn show_count(&self) {
        print!("\n\n  t2 result = {:?}",self.counter.lock().unwrap());
    }
}

fn main() {

    print!("\n  -- demo Test1a --");
    let mut t1a = Test1a::new();
    let handle = t1a.start();
    let _ = handle.join();
    t1a.show_count();
    println!();

    print!("\n  -- demo Test2 --");
    let (t2, handle) = Test2::new();
    let _ = handle.join();
    t2.show_count();

    println!("\n\n  That's all Folks!\n\n");
}
