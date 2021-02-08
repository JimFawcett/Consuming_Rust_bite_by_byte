/*
  iterators provide next function that
  returns an Option containing Some(item)
  if there is a next item, otherwise None
*/
fn do_iter(v: &Vec<i32>) {
    print!("\n  ");
    let mut iter = v.iter();
    loop {
        let item = iter.next();
        if item.is_none() {
            print!("{:?}", item);
            break;
        }
        print!("{:?} ", item);
    }
}
/*------------------------------------
  Demonstrate idiomatic iteration
  for loop iterates and unwraps Option
*/
fn do_idiomatic_iter(v: &Vec<i32>) { 
    print!("\n  ");
    for item in v {
        print!("{:?} ", item);
    }
}

fn main() {
    do_iter(&vec![1, 2, 3]);
    do_idiomatic_iter(&vec![1, 2, 3]);
}