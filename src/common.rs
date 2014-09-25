pub struct FactorIterator {
    value: uint,
    current: uint,
    max: uint,
}

impl FactorIterator {
    pub fn new(value: uint, limit: uint) -> FactorIterator {
        FactorIterator {
            current: 0u,
            value: value,
            max: limit,
        }
    }
}

impl Iterator<uint> for FactorIterator {
    fn next(&mut self) -> Option<uint> {
        self.current += 1;
        while self.current < self.max {
            if self.current % 1_000_000 == 0 {
                println!("At {}", self.current);
            }
            if self.value % self.current == 0 {
                return Some(self.current)
            }
            self.current += 1;
        }
        None
    }
}

pub fn is_prime(value: uint) -> bool {
    if value == 1 {
        return false;
    }
    for i in range(2, value) {
        if value % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::{FactorIterator, is_prime};

    #[test]
    fn test_factorization() {
        let mut iter = FactorIterator::new(22, 22);
        let result: Vec<uint> = iter.collect();
        assert_eq!(result, vec![1, 2, 11]);
    }

    #[test]
    fn test_primality() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
    }
}
