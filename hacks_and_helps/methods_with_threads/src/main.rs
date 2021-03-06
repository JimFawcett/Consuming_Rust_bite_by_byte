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
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

struct Test1 {
    count: Arc<Mutex<i32>>,
}
impl Test1 {
    fn new() -> Test1 {
        Test1 {
            count: Arc::new(Mutex::new(0)),
        }
    }
    fn start(&mut self) -> JoinHandle<()> {
        let local_count = self.count.clone();  // get a shared ptr
        std::thread::spawn(move || {
            for i in 0..5 {
                let mut data = local_count.lock().unwrap();
                *data += i;
                print!("\n  {:?}", data);
            }
        })
    }
    fn show_count(&self) {
        print!("\n\n  t1 result = {:?}",self.count.lock().unwrap());
    }
}

struct Test2 {
    count: Arc<Mutex<i32>>,
}
impl Test2 {
    fn new() -> Test2 {
        Test2 {
            count: Arc::new(Mutex::new(0)),
        }
    }
    /*-----------------------------------------------------
      Instead of using member data in Test2::start we've
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
        print!("\n\n  t2 result = {:?}",self.count.lock().unwrap());
    }
}

struct Test3 {
    counter: Arc<Mutex<i32>>,
}
impl Test3 {
    /*-----------------------------------------------------
      This example uses the same technique as Test2, but
      moves the thread into new(), the Test2 constructor.  
      That would be a good idea for any type whose instances 
      need the running thread to operate as expected. 

      Note that new() now returns a tuple with the newly
      constructed Test2 instance and the thread handle.
      Look at main to see how that is used.
    */
    fn new() -> (Test3, JoinHandle<()>) {
        let scount = Arc::new(Mutex::new(0));  // initial value is 0
        let share = Arc::clone(&scount);
        let handle = std::thread::spawn(move || {  // scount moved
            for i in 0..5 {
                let mut data = scount.lock().unwrap();
                *data += i;
                print!("\n  {:?}", data);
            }
        });
        (
            Test3 { counter: share, },  
            handle
        )
    }
    fn show_count(&self) {
        print!("\n\n  t3 result = {:?}",self.counter.lock().unwrap());
    }
}

struct Test4 {
    counter: Arc<Mutex<i32>>,
    do_run : Arc<AtomicBool>,
}
impl Test4 {
    /*-----------------------------------------------------
      This example demonstrates graceful thread shutdown
      using an AtomicBool, set by the user and tested in 
      the thread loop.
    */
    fn new() -> (Test4, JoinHandle<()>) {
        let scount = Arc::new(Mutex::new(0));
        let share = Arc::clone(&scount);
        let run = Arc::new(AtomicBool::new(true));
        let run_clone = Arc::clone(&run);
        let handle = std::thread::spawn(move || {  // scount moved
            for i in 0..5000 {
                if !run.load(Ordering::Relaxed) {
                    break;
                }
                /* slow down loop for display */
                std::thread::sleep(Duration::from_micros(200));
                let mut data = scount.lock().unwrap();
                *data = i;
                print!("\n  {:?}", data);
            }
        });
        (
            Test4 { counter: share, do_run: run_clone },  
            handle
        )
    }
    fn stop(&self) {
        self.do_run.store(false, Ordering::Relaxed);
    } 
    fn show_count(&self) {
        print!("\n\n  t4 result = {:?}",self.counter.lock().unwrap());
    }
}

fn main() {

    print!("\n  -- demo Test1 - uses start() method --");
    let mut t1 = Test1::new();
    let handle = t1.start();
    let _ = handle.join();
    t1.show_count();
    println!();

    print!("\n  -- demo Test2 --");
    let mut t2 = Test2::new();
    let handle = t2.start();
    let _ = handle.join();
    t2.show_count();
    println!();

    print!("\n  -- demo Test3 - thread started in new() --");
    let (t3, handle) = Test3::new();
    let _ = handle.join();
    t3.show_count();
    println!();

    print!("\n  -- demo Test4 - graceful stop --");
    let (t4, handle) = Test4::new();
    std::thread::sleep(Duration::from_millis(100));
    t4.stop();
    let _ = handle.join();
    t4.show_count();

    println!("\n\n  That's all Folks!\n\n");
}
