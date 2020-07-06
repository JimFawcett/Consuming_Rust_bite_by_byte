// closure_param::main.rs

fn take_closure<F>(f:F) where F:FnOnce() -> bool {
  if f() {
      print!("\n  closure returned true");
  }
  else {
      print!("\n  closure returned false");
  }
}

fn main() {
    let s = "closure data".to_string();
    let p = false;
    let c = || {
        print!("\n  closure string = {:?}", s);
        p
    };

    take_closure(c);

    println!("\n\n  That's all Folks!\n\n");
}
