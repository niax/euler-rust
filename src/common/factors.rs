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

    pub fn default_max(value: uint) -> FactorIterator {
        let max = (value as f64).sqrt().ceil();
        FactorIterator::new(value, max as uint)
    }
}

impl Iterator<uint> for FactorIterator {
    fn next(&mut self) -> Option<uint> {
        self.current += 1;
        while self.current <= self.max {
            if self.value % self.current == 0 {
                return Some(self.current)
            }
            self.current += 1;
        }
        None
    }
}



#[cfg(test)]
mod test {
    use super::{FactorIterator};

    #[test]
    fn test_factorization() {
        let mut iter = FactorIterator::new(22, 22);
        let result: Vec<uint> = iter.collect();
        assert_eq!(result, vec![1, 2, 11, 22]);
    }
}
