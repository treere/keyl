mod input;

fn main() {
    let device = std::env::args().nth(1).expect("Missing device name");

    let mut k = input::Keyboard::new(device).expect("Cannot open input");

    loop {
        println!("{:?}", k.key());
    }
}
