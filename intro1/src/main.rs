#![allow(dead_code)]
use core::fmt::Debug;

/*-------------------------------------------------
  Show slice as stack of rows with span elements
  in row
  - nice illustration of Iterator methods
*/
fn show_fold<T:Debug>(t:&[T], span:usize/*, width:usize*/) {
    let times = 1 + t.len()/span;
    let iter = t.iter();
    print!("\n  ");
    for _i in 0..times {
        for bt in iter.clone()
            .skip(_i * span).take(span) {
            print!("{:5?} ", bt);
        }
        if _i < times - 1 {
            print!("\n  ");
        }
    }
}

fn get_type<T>(_t:&T) -> &str {
    std::any::type_name::<T>()
}

fn show_type_value<T: Debug>(msg: &str, t: &T) {
    print!(
        "\n  {} type is: {}, value: {:?}", 
        msg, get_type::<T>(t), t
    );
}
 
fn main() {
  print!("\n  -- demo_string --");
  let s1 : String = String::from("a test string");
  show_type_value("s1 - ", &s1);
  print!("\n  -- iterating through String characters --");
  let iter = s1.chars();
  print!("\n  ");
  for ch in iter {
      print!("{} ", ch);
  }
  print!("\n  -- extracting bytes --");
  let s1_bytes = s1.as_bytes();
  print!("\n  bytes are:");
  show_fold(&s1_bytes, 5);
  // This works too, will wrap in []
  // print!("\n  bytes are: {:?}", b"a test string");
  
  print!("\n  -- extracting a slice --");
  let slc = &s1[0..6];
  show_type_value("&s1[0..6]", &slc);
  print!("\n  -- demonstrate move --");
  print!("\n  executing statement: let s2 = s1;");
  print!("\n  address of s1    = {:p}", &s1);
  print!(
        "\n  address of s1.as_bytes()[0] = {:p}", 
        &s1.as_bytes()[0]
  );
  let s2 = s1;
  print!("\n  address of s2    = {:p}", &s2);
  print!(
      "\n  address of s2.as_bytes()[0] = {:p}", 
      &s2.as_bytes()[0]
  );
  print!("\n  new control block, original start of heap alloc");
}