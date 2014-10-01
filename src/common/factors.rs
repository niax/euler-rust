use std::collections::HashMap;
use common::primes::{PrimeIterator, is_prime};

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

fn prime_factors_recurse(n: uint, prime_seed: &PrimeIterator, factors: &mut HashMap<uint, uint>) {
    let mut prime_iter = PrimeIterator::new_from_seed(prime_seed);
    let new_factor = if is_prime(n) {
        n
    } else {
        let mut fact = 0u;
        for i in prime_iter {
            if n % i == 0 {
                prime_factors_recurse(n / i, prime_seed, factors);
                fact = i;
                break;
            }
        }
        fact
    };

    let new_count = match factors.find(&new_factor) {
        Some(old_count) => old_count + 1,
        None => 1
    };
    factors.insert(new_factor, new_count);
}

pub fn prime_factors(n: uint, prime_seed: &PrimeIterator) -> HashMap<uint, uint> {
    let mut factors: HashMap<uint, uint> = HashMap::new();

    prime_factors_recurse(n, prime_seed, &mut factors);

    factors
}


#[cfg(test)]
mod test {
    use super::{FactorIterator, prime_factors};
    use common::primes::PrimeIterator;
    use std::collections::HashMap;

    #[test]
    fn test_factorization() {
        let mut iter = FactorIterator::new(22, 22);
        let result: Vec<uint> = iter.collect();
        assert_eq!(result, vec![1, 2, 11, 22]);
    }


    #[test]
    fn test_prime_factors() {
        let prime_seed = PrimeIterator::new_with_size(1024);
        let factors = prime_factors(864, &prime_seed);
        let mut expected = HashMap::new();
        expected.insert(2u, 5u);
        expected.insert(3u, 3u);
        assert_eq!(factors, expected);
    }
}
