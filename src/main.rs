extern crate notify_rust;
extern crate rand;

use notify_rust::Notification;
use std::{thread, time};
use std::env;
use rand::Rng;

// the interval between notification showing
const MINUTES: u64 = 25;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut minutes = MINUTES;
    if arguments.len() > 1 {
        minutes = arguments[1].parse::<u64>().unwrap();
    }
    println!("Timegone started. Interval set to {} minutes", minutes);
    let interval = time::Duration::from_secs(minutes * 60);
    loop {
        thread::sleep(interval);

        Notification::new()
            .summary("Timegone")
            .body(&get_message())
            .show()
            .unwrap();
    }
}

fn get_message() -> String {
    let messages: Vec<String> = [
        "Stand Up and walk around now",
        "Go ahead",
        "Stop looking at me",
        "Look out the window",
        "Time to walk!",
        "Right time to some workout"
    ]
        .iter()
        .map(|&s| s.into())
        .collect();

    let index: usize = rand::thread_rng().gen_range(0, messages.len());

    messages[index].to_string()
}
