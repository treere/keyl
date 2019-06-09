use std::collections::HashMap;
use crate::input::{KeyValue, KeyCode, KeyEvent};

mod input;


pub struct KeyBalancer {
    hash: HashMap<KeyCode, KeyValue>
}

impl KeyBalancer {
    pub fn new() -> KeyBalancer {
        KeyBalancer { hash: HashMap::new() }
    }
    pub fn add(&mut self, k: KeyEvent) -> Option<KeyEvent> {
        match k.value {
            0 => {
                if let Some(v) = self.hash.get(&k.code) {
                    let value = *v;
                    self.hash.remove(&k.code);
                    Some(KeyEvent { code: k.code, value })
                } else {
                    None
                }
            },
            1 | 2 => {
                self.hash.insert(k.code, k.value);
                None
            }
            _ => unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invariant_zero() {
        let mut k = KeyBalancer::new();
        let zero = KeyEvent { code: 42, value: 0 };
        assert_eq!(None, k.add(zero.clone()));
        assert_eq!(None, k.add(zero.clone()));
    }

    #[test]
    fn add_one() {
        let mut k = KeyBalancer::new();
        let zero = KeyEvent { code: 42, value: 0 };
        let one = KeyEvent { code: 42, value: 1 };
        assert_eq!(None, k.add(one.clone()));
        assert_eq!(Some(one), k.add(zero.clone()));
    }

    #[test]
    fn add_one_and_two() {
        let mut k = KeyBalancer::new();
        let zero = KeyEvent { code: 42, value: 0 };
        let one = KeyEvent { code: 42, value: 1 };
        let two = KeyEvent { code: 42, value: 2 };
        assert_eq!(None, k.add(one.clone()));
        assert_eq!(None, k.add(two.clone()));
        assert_eq!(Some(two), k.add(zero.clone()));
    }

    #[test]
    fn zero_repetition() {
        let mut k = KeyBalancer::new();
        let zero = KeyEvent { code: 42, value: 0 };
        let one = KeyEvent { code: 42, value: 1 };
        assert_eq!(None, k.add(one.clone()));
        assert_eq!(Some(one), k.add(zero.clone()));
        assert_eq!(None, k.add(zero.clone()));
    }
}


fn main() {
    let device = std::env::args().nth(1).expect("Missing device name");

    let mut k = input::Keyboard::new(device).expect("Cannot open input");

    let mut kb = KeyBalancer::new();


    loop {
        if let Some(x) = kb.add(k.key()) {
            println!("{:?}", x);
        }
    }
}
