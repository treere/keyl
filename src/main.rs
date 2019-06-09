
mod input;

mod balancer;

fn main() {
    let device = std::env::args().nth(1).expect("Missing device name");

    let mut k = input::Keyboard::new(device).expect("Cannot open input");

    let mut kb = balancer::KeyBalancer::new();


    loop {
        if let Some(x) = kb.add(k.key()) {
            println!("{:?}", x);
        }
    }
}
