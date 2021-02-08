use std::fmt::*;
use std::collections::HashMap;

fn type_is<T>(_t:T) -> String {
    format!("{}", std::any::type_name::<T>())
}
/*----------------------------------------
  - Pass by ref to avoid moving t into
    function.
  - that prepends & char to type
  - remove first & char from type
    string by taking slice
*/
fn show_type<T>(t:&T) {
    let s = type_is(t);
    let s = &s[1..];  // remove leading & char
    print!("\n  type is {}", s);
}
/*--------------------------------------
  Demonstrate basic iteration
*/
fn do_iter<T>(iter: &mut T) 
  where T: Iterator + Debug, T::Item: Debug {
    print!("\n  ");
    loop {
        let item = iter.next();
        if item.is_none() {
            break;
        }
        print!("{:?} ", item.unwrap());
    }
}
/*------------------------------------
  Demonstrate idiomatic iteration
*/
fn do_idiomatic_iter<T>(t:T) 
  where T: IntoIterator, T::Item: Debug {
    print!("\n  ");
    for item in t {
        print!("{:?} ", item);
    }
}

fn main() {
    let putln = || println!();
    let putmsg = |msg:&str| print!("\n  {}", msg);
    
    putmsg("-- iterating over array --");
    let arr = [1, 2, 3];
    show_type(&arr);
    do_iter(&mut arr.iter());
    do_idiomatic_iter(&arr);
    putln();
    
    putmsg("-- iterating over String --");
    let s = "a string".to_string();
    show_type(&s);
    do_iter(&mut s.chars());
    /* String doesn't implement IntoIterator */
    do_idiomatic_iter(s.bytes());
    putln();
    
    putmsg("-- iterating over String slice --");
    let slice = &s[2..];
    show_type(&slice);
    do_iter(&mut slice.char_indices());
    /* &str doesn't implement IntoIterator */
    do_idiomatic_iter(slice.bytes());
    putln();

    putmsg("-- iterating over byte array --");
    let bytes = &s[2..].as_bytes();
    show_type(&bytes);
    do_iter(&mut bytes.iter());
    do_idiomatic_iter(&**bytes);
    putln();
    
    putmsg("-- iterating over Vec --");
    let v = vec![1, 2, 3];
    show_type(&v);
    do_iter(&mut v.iter());
    do_idiomatic_iter(&v);
    putln();
    
    putmsg("-- iterating over HashMap --");
    let mut h = HashMap::<String, i32>::new();
    h.insert("zero".to_string(), 0);
    h.insert("one".to_string(), 1);
    h.insert("two".to_string(), 2);
    h.insert("three".to_string(), 3);
    show_type(&h);
    do_iter(&mut h.iter());
    do_idiomatic_iter(&h);
    
    print!("\n\n  That's all Folks\n\n");
}