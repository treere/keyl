use std::collections::HashMap;
use crate::input::KeyCode;

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
            }
        };
    }
}

#[cfg(test)]
mod tests {
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
