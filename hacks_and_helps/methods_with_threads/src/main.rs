// methods_with_threads

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

struct Test2 {
    counter: Arc<Mutex<i32>>,
}
impl Test2 {
    /*-----------------------------------------------------
      This works because this thread owns scount.  A shared
      reference is given to the instance at the end of this
      new function.  
    */
    fn new() -> (Test2, JoinHandle<()>) {
        let scount = Arc::new(Mutex::new(0));  // moved into thread
        let share = Arc::clone(&scount);
        let handle = std::thread::spawn(move || {
            for i in 0..5 {
                let mut data = scount.lock().unwrap();
                *data += i;
                print!("\n  {:?}", data);
            }
        });

        // To demonstrate that share, below, is still valid after 
        // thread completes uncomment the code line below.

        // std::thread::sleep(Duration::from_millis(5000));

        // scount is invalid in this scope (been moved into thread), 
        // but share is valid ref to value also referenced by scount,
        // since an Arc's value is not dropped until all shared
        // references are dropped.
        (
            Test2 { counter: share, },  
            handle
        )
    }
    fn show_count(&self) {
        print!("\n\n  t2 result = {:?}",self.counter.lock().unwrap());
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
      self.count at the end, e.g., same idea as Test2. 
    */
    fn start(&mut self) -> JoinHandle<()> {
        let scount = Arc::new(Mutex::new(0));  // moved into thread
        let share = Arc::clone(&scount);
        let handle = std::thread::spawn(move || {
            for i in 0..5 {
                let mut data = scount.lock().unwrap();
                *data += i;
                print!("\n  {:?}", data);
                /* data unlocked here*/
            }
        });
        self.count = share;
        handle
    }
    fn show_count(&self) {
        print!("\n\n  t1a result = {:?}",self.count.lock().unwrap());
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
