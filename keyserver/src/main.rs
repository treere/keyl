#![feature(deadline_api)]

use keyllib::*;
use std::sync::{Arc, Mutex};
use std::{fs, io};
use hyper::service::{service_fn, service_fn_ok};
use hyper::{Request, Response, header, Body};

fn main() -> std::io::Result<()> {
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

    let vec = Arc::new(Mutex::new(Vec::new()));

    let v = vec.clone();
    let _t = std::thread::spawn(move || {
        loop {
            let end = std::time::Instant::now() + std::time::Duration::from_secs(60);
            let mut st = stat::KeyEventCounter::new();
            loop {
                match rx.recv_deadline(end) {
                    Ok(p) => { st.insert(p); }
                    Err(_) => break,
                };
            }
            v.lock().map(|mut v| v.push((chrono::Local::now(), st.count_all()))).expect("ok");
        }
    });

    let path = "/tmp/test.sock";
    if let Err(err) = fs::remove_file(path) {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err);
        }
    }
    let v = vec.clone();
    let service = move || {
        let v = v.clone();
        service_fn_ok(move |_: Request<Body>| {
            let s = v.lock().map(|v| {
                if let Some((time, (s, l))) = v.last() {
                    format!("{},{},{}", time, s, l)
                } else {
                    "".to_owned()
                }
            }).unwrap();
            let t: Response<Body> = Response::builder()
                .header(header::CONTENT_TYPE, "text/plain")
                .header(header::CONTENT_LENGTH, s.len() as u64)
                .body(s.into())
                .expect("failed to create response");
            t
        })
    };
    let svr = hyperlocal::server::Server::bind(path, service)?;

    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(path, perms)?;

    println!("Listening on unix://{path} with 1 thread.", path = path);
    svr.run()?;
    Ok(())
}
