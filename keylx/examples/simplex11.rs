#![feature(deadline_api)]

use keylx::hardware::*;
use std::collections::HashMap;
use std::time::SystemTime;

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

    let mut m = HashMap::new();
    let d = std::time::Duration::from_secs(10);
    loop {
        let t = std::time::Instant::now();
        let end = t + d;
        loop {
            match rx.recv_deadline(end) {
                Ok(name) => {
                    let value = *m.get(&name).unwrap_or(&0);
                    m.insert(name, value + 1);
                }
                Err(_) => break,
            };
        }
        let total = m.values().sum::<u32>();

        m.insert("__total__", total);

        // after 2100?
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Cannot compute time").as_secs() as u32;
        m.insert("__time__", time);
        m.insert("__duration__", d.as_secs() as u32);

        println!("{:?}", m);
        m.clear();
    }
}
