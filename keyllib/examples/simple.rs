#![feature(deadline_api)]

use keyllib::*;

fn main() {
    let device = std::env::args().nth(1).expect("Missing device name");

    let (sx, rx) = std::sync::mpsc::channel();

    let mut k = input::Keyboard::new(device).expect("Cannot open input");

    let mut kb = balancer::KeyBalancer::new();
    let _t = std::thread::spawn(move || {
        loop {
            if let Some(x) = kb.add(k.key()) {
                sx.send(x).expect("Error in send")
            }
        }
    });

    loop {
        let t = std::time::Instant::now();
        let d = std::time::Duration::from_secs(60);
        let end = t + d;
        let mut st = stat::KeyEventCounter::new();
        loop {
            match rx.recv_deadline(end) {
                Ok(p) => { st.insert(p); },
                Err(_) => break,
            };
        }
        let (short, long) = st.count_all();
        println!("{}: s {}\tl {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), short, long);
    }
}
