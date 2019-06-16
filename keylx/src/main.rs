#![feature(deadline_api)]

use keyllib::hardware::*;
use keyllib::KeystrokeStorage;
use clap::{App, Arg};
use std::time::Duration;

fn main() {
    let matches = App::new("KeylX")
        .version("0.1")
        .author("Andrea Tomasi <tomasiandrea.at@gmail.com>")
        .about("Collect keystrokes in an interval and print a json the the statistics")
        .arg(Arg::with_name("inteval")
            .short("i")
            .long("inteval")
            .value_name("SECONDS")
            .help("Sampling interval"))
        .get_matches();

    let interval = {
        let secs = matches.value_of("interval").and_then(|x| x.parse().ok()).unwrap_or(60);
        Duration::from_secs(secs)
    };

    let (sx, rx) = std::sync::mpsc::channel();

    let _ = std::thread::spawn(move || {
        let mut keyboard = XKeyboard::new(XDisplay::new());
        loop {
            if let Some(name) = keyboard.keyname() {
                sx.send(name).expect("Error in sending");
            }
        }
    });

    let mut storage = KeystrokeStorage::new();
    loop {
        let end = std::time::Instant::now() + interval;
        loop {
            match rx.recv_deadline(end) {
                Ok(name) => storage.insert(name),
                Err(_) => break,
            };
        }

        println!("{}", storage.complete_to_string(&interval))
    }
}
