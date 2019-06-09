mod input;

mod balancer;

mod stat;

fn main() {
    let device = std::env::args().nth(1).expect("Missing device name");

    let mut k = input::Keyboard::new(device).expect("Cannot open input");

    let mut kb = balancer::KeyBalancer::new();

    let mut st = stat::KeyPressCounter::new();


    for _ in 0..100 {

        if let Some(x) = kb.add(k.key()) {
            st.insert(x.code);
        }
    }
    println!("{:?}", st);
}
