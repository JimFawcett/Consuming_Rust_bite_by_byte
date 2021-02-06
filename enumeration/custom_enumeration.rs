#![allow(unused_variables)]

#[derive(Debug, Clone)]
enum Event<T> { Normal(T), Warning(T), Critical(T), NoEvent }
impl<T> Event<T> {
    fn unwrap(&self) -> &T {
        if let Event::Normal(ev) = self{ ev } 
        else if let Event::Warning(ev) = self{ev}
        else if let Event::Critical(ev) = self{ev}
        else { panic!() }
    }
}

use Event::*;

fn main() {
    let e1: Event<u8> = Event::<u8>::Normal(1);
    let e2 = Warning(2);
    let e3 = Critical(3);
    let e4: Event<u8> = NoEvent;
    
    match e3 {
        Normal(ev) => print!("\n  event {} is Normal", ev),
        Warning(ev) => print!("\n  event {} is Warning", ev),
        Critical(ev) => print!("\n  event {} is Critical!", ev),
        NoEvent => print!("\n  no events occurred"),
    }
    
    let e = e2.clone();
    
    if let Warning(ev) = e {
        print!("\n  event {} is Warning", ev);
    }
    
    let v = e3.unwrap();
    print!("\n  inner value of {:?} is {}", e3, v);
}