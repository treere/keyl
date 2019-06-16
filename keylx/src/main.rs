#![feature(deadline_api)]

use keyllib::hardware::*;
use keyllib::KeystrokeStorage;

fn main() {
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
    let d = std::time::Duration::from_secs(10);
    loop {
        let t = std::time::Instant::now();
        let end = t + d;
        loop {
            match rx.recv_deadline(end) {
                Ok(name) => storage.insert(name),
                Err(_) => break,
            };
        }

        println!("{}", storage.complete_to_string(&d))
    }
}
