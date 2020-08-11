/////////////////////////////////////////////////////////////
// hacks_and_helps::main.rs - dates, times, and timers     //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 10 Jul 2020  //
/////////////////////////////////////////////////////////////

extern crate chrono;

/*-----------------------------------------------
  StopWatch type measures elapsed times
*/
use std::time::*;
use std::{thread};
use std::thread::{JoinHandle};
use std::fmt::*;

#[derive(Debug, Clone, Copy)]
pub struct StopWatch {
    start: Instant,
    elapsed: Duration,
}
impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            start: Instant::now(),
            elapsed: Duration::new(0,0),
        }
    }
    pub fn start(&mut self) {
        self.start = Instant::now();
    }
    pub fn stop(&mut self) -> Duration {
        self.elapsed = self.start.elapsed();
        self.elapsed
    }
    pub fn elapsed_micros(&self) -> u128  {
        self.elapsed.as_micros()
    }
    pub fn elapsed_millis(&self) -> u128 {
        self.elapsed.as_millis()
    }
    pub fn elapsed_secs(&self) -> u64 {
        self.elapsed.as_secs()
    }
}

fn sleep(millisec:u64) {
    let secs = Duration::from_millis(millisec);
    thread::sleep(secs);
}
/*-----------------------------------------------
  stop_watch displays thread sleep time
  - expect small variations from run to run
    due to uncertainty in sleep interval
*/
fn stop_watch(millisec: u64) {
    let mut sw = StopWatch::new();
    sleep(millisec);
    let time = sw.stop().as_millis();
    print!("\n  elapsed time = {:?}",time);
}
/*-----------------------------------------------
  Timer instance invokes callback after 
  specified time.
  - callback runs on Timer thread
*/
#[derive(Debug, Clone, Copy)]
struct Timer {
    start: Instant,
    elapsed: Duration,
}
impl Timer {
    pub fn new(time: u64) -> Timer {
        Timer {
            start: Instant::now(),
            elapsed: Duration::new(time, 0),
        }
    }
    pub fn start<F>(&mut self, time: u64, callback:F) -> JoinHandle<()>
        where F:FnOnce() + Send + 'static {
        self.start = Instant::now();
        let handle = thread::spawn(move || {
            let ttw = Duration::from_millis(time);
            thread::sleep(ttw);
            callback();
        });
        handle
    }
}
/*-----------------------------------------------
  Demo Timer, using closure callback
*/
fn timer(millisec: u64) -> JoinHandle<()> {
    let mut tmr = Timer::new(millisec);
    let cl = move || { 
        print!("\n  time's up after {:?} milliseconds", millisec) 
    };
    tmr.start(millisec, cl)
}
/*-----------------------------------------------
  Date-Time stamp
*/
#[allow(unused_imports)]
use chrono::{DateTime, Local, Datelike, Timelike};

pub fn date_time_stamp() -> String {
    let now: DateTime<Local> = Local::now();
    /* format DateTime string using chrono formatting */
    let mut now_str = format!("{}", now.to_rfc2822());
    /* remove trailing -0400 */
    now_str.truncate(now_str.len() - 6);
    now_str
}

fn convert_month(m:usize) -> &'static str {
    let dv = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];
    dv[m]
}
pub fn date_stamp() -> String {
    let now: DateTime<Local> = Local::now();
    /* format date string */
    let (_is_common_era, year) = now.year_ce();
    let idx:usize = (now.month() - 1) as usize;
    let mon = convert_month(idx);
    let now_str = format!(
        "{:0>2} {} {}", 
        now.day(), mon, year
    );
    now_str
}
pub fn time_stamp() -> String {
    let now: DateTime<Local> = Local::now();
    /* format time string */ 
    let now_str = format!(
        "{:0>2}:{:0>2}:{:0>2}", 
        now.hour(), now.minute(), now.second()
    );
    now_str
}

/*-----------------------------------------------
  Demonstrations of StopWatch, Timer, ...
*/
fn main() {
    print!("\n  === demo date_time_timer ===");
    println!();
    
    print!("\n  -- demo StopWatch --");
    stop_watch(25);
    println!();
    
    print!("\n  -- demo Timer --");
    print!("\n  starting timer(200)");
    let handle = timer(200);
    print!("\n  do some work while waiting for timer");
    let _ = handle.join(); 
    println!();
    
    print!("\n  -- demo DateTimeStamp --");
    print!("\n  now is:  {:?}", date_time_stamp());
    print!("\n  date is: {:?}", date_stamp());
    print!("\n  time is: {:?}", time_stamp());
    println!("\n\n  That's all Folks!\n\n");
}