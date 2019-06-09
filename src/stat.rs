use std::collections::HashMap;
use crate::input::{KeyCode, KeyEvent};


#[derive(Debug)]
pub struct KeyPressCounter {
    counts: HashMap<KeyCode, usize>
}

impl KeyPressCounter {
    pub fn new() -> KeyPressCounter {
        KeyPressCounter { counts: HashMap::new() }
    }

    pub fn count(&self, k: KeyCode) -> usize {
        *self.counts.get(&k).unwrap_or(&0)
    }

    pub fn insert(&mut self, k: KeyCode) {
        match self.counts.get(&k) {
            None => self.counts.insert(k, 1),
            Some(x) => {
                let value = *x;
                self.counts.insert(k, value + 1)
            },
        };
    }
}

#[cfg(test)]
mod key_press_counter_tests {
    use super::*;

    #[test]
    fn none() {
        let k = KeyPressCounter::new();

        assert_eq!(0, k.count(9))
    }
    #[test]
    fn one() {
        let mut k = KeyPressCounter::new();
        k.insert(9);

        assert_eq!(1, k.count(9))
    }
    #[test]
    fn three() {
        let mut k = KeyPressCounter::new();
        k.insert(9);
        k.insert(9);
        k.insert(9);

        assert_eq!(3, k.count(9))
    }
    #[test]
    fn different() {
        let mut k = KeyPressCounter::new();
        k.insert(19);
        k.insert(9);
        k.insert(19);

        assert_eq!(1, k.count(9));
        assert_eq!(2, k.count(19));
    }
}

#[derive(Debug)]
pub struct KeyEventCounter {
    short: KeyPressCounter,
    long: KeyPressCounter,
}

impl KeyEventCounter {
    pub fn new() -> KeyEventCounter {
        KeyEventCounter {
            short: KeyPressCounter::new(),
            long: KeyPressCounter::new(),
        }
    }
    pub fn count(&self, k: KeyCode) -> (usize, usize) {
        (self.short.count(k), self.long.count(k))
    }

    pub fn insert(&mut self, k: KeyEvent) {
        match k.value {
            1 => self.short.insert(k.code),
            2 => self.long.insert(k.code),
            _ => unimplemented!()
        }
    }
}

#[cfg(test)]
mod key_event_counter_tests {
    use super::*;

    #[test]
    fn none() {
        let k = KeyEventCounter::new();
        assert_eq!((0, 0), k.count(42))
    }

    #[test]
    fn short() {
        let mut k = KeyEventCounter::new();
        k.insert(KeyEvent { code: 42, value: 1 });

        assert_eq!((1, 0), k.count(42))
    }

    #[test]
    fn long() {
        let mut k = KeyEventCounter::new();
        k.insert(KeyEvent { code: 42, value: 2 });

        assert_eq!((0, 1), k.count(42))
    }

    #[test]
    fn multiple() {
        let mut k = KeyEventCounter::new();
        k.insert(KeyEvent { code: 42, value: 1 });
        k.insert(KeyEvent { code: 42, value: 2 });
        k.insert(KeyEvent { code: 42, value: 1 });

        assert_eq!((2, 1), k.count(42))
    }
}

