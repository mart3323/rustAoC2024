mod day0;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;
mod day6;

use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use crate::day0::solve_day0;
use crate::day2::solve_day2;
use crate::day3::solve_day3;
use crate::day4::solve_day4;
use crate::day5::solve_day5;
use crate::day6::solve_day6;

fn benchmark(function: fn(), name: &str) -> usize {
    let stop = channel();
    let count = Arc::new(AtomicUsize::new(0));
    let innercount = Arc::clone(&count);
    let handle = thread::spawn(move || {
        loop {
            function();
            if let Ok(_) = stop.1.try_recv() {
                return
            }
            innercount.fetch_add(1, Relaxed);
        }
    });
    thread::sleep(Duration::from_secs(5));
    if let Err(e) = stop.0.send(()) {
        panic!("Couldn't send stop signal to stop the thread: {}", e);
    }
    if let Err(e) = handle.join() {
        panic!("Couldn't join thread (we want to wait for it to stop): {:?}", e);
    }
    return count.load(Relaxed);
}

fn main() {
    solve_day0();
    solve_day2();
    solve_day3();
    solve_day4();
    solve_day5();
    solve_day6();
}

fn bench() {
    let time6 = benchmark(solve_day6, "day6");
    let time5 = benchmark(solve_day5, "day5");
    let time4 = benchmark(solve_day5, "day4");
    let time3 = benchmark(solve_day3, "day3");
    let time2 = benchmark(solve_day2, "day2");
    let time0 = benchmark(solve_day0, "day0");

    println!("time6: {}", time6);
    println!("time5: {}", time5);
    println!("time4: {}", time4);
    println!("time3: {}", time3);
    println!("time2: {}", time2);
    println!("time0: {}", time0);
}
