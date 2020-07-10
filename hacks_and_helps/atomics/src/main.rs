/////////////////////////////////////////////////////////////
// atomics::main.rs - demo AtomicBool and AtomicUsize      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 10 Jul 2020  //
/////////////////////////////////////////////////////////////

use std::sync::atomic::*;
use std::sync::*;

/*-- thread-safe bool operations --*/
fn basic_test_atomic_bool() {
    print!("\n  -- demo basic AtomicBool operations --");
    let mut abool = AtomicBool::new(true);
    assert_eq!(*abool.get_mut(),true);
    *abool.get_mut() = false;
    assert_eq!(*abool.get_mut(),false);
    print!("\n  if you see this everything worked!");
}

/*-- sharing atomic bool --*/
fn test_shared_atomic_bool() {
    print!("\n  -- demo sharing AtomicBool between threads --");
    let shared_bool = Arc::new(AtomicBool::new(true));
    let shared_bool1 = Arc::clone(&shared_bool);
    let shared_bool2 = Arc::clone(&shared_bool);
    print!("\n  value of atomic bool is {:?}", shared_bool1.as_ref());
    let handle = std::thread::spawn(move || {
        shared_bool2.store(false, Ordering::Relaxed);
    });
    let _ = handle.join();
    shared_bool1.load(Ordering::Relaxed);
    print!("\n  value of atomic bool is {:?}", shared_bool1.as_ref());
}

/*-- sharing atomic bool: one thread increms, one prints --*/
fn test_shared_atomic_usize() {
    print!("\n  -- demo sharing AtomicUsize between threads --");
    let shared_usize = Arc::new(AtomicUsize::new(0));
    let shared_usize1 = Arc::clone(&shared_usize);
    let shared_usize2 = Arc::clone(&shared_usize);
    print!("\n  starting value of atomic usize is {:?}", shared_usize1.as_ref());
    /*-- thread increments shared usize --*/
    let handle1 = std::thread::spawn(move || {
        let ms = std::time::Duration::from_millis(80);
        for i in 0..10 {
            shared_usize1.store(i, Ordering::Relaxed);
            std::thread::sleep(ms);
        }
    });
    /*-- thread displays shared usize --*/
    let handle2 = std::thread::spawn(move || {
        /* inc every 80 ms, print every 100ms --*/
        let ms = std::time::Duration::from_millis(100);
        for _i in 0..10 {
            shared_usize2.load(Ordering::Relaxed);
            print!("\n  value of atomic usize is {:?}", shared_usize2.as_ref());
            std::thread::sleep(ms);
        }
    });
    let _ = handle1.join();
    let _ = handle2.join();
    shared_usize.load(Ordering::Relaxed);
    print!("\n  value of final atomic usize is {:?}", shared_usize.as_ref());
}

fn main() {

    print!("\n  === Demonstrate Sharing Atomics between Threads ===");
    println!();

    basic_test_atomic_bool();
    println!();

    test_shared_atomic_bool();
    println!();

    test_shared_atomic_usize();

    println!("\n\n  That's all Folks!\n\n");
}
