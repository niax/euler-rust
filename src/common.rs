use std::iter::range_step;
use std::collections::Bitv;

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

/// Calculates the greatest common divisor of `a` and `b`
pub fn gcd(a: uint, b: uint) -> uint {
    let mut i = a;
    let mut j = b;
    while i != j {
        if i > j {
            i = i - j;
        } else {
            j = j - i;
        }
    }
    i
}

/// Calculates the lowest common multiple of `a` and `b`
pub fn lcm(a: uint, b: uint) -> uint {
    if a == 0 && b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

/// Iterator over prime numbers from 2
pub struct PrimeIterator {
    current: uint,
    bitv: Bitv,
}


impl PrimeIterator {
    pub fn new() -> PrimeIterator {
        let mut initial_vector = Bitv::with_capacity(8, false);
        // Preset the initial vector with primes
        for i in vec![2, 3, 5, 7].iter() {
            initial_vector.set(*i, true);
        }
        PrimeIterator {
            current: 0,
            bitv: initial_vector,
        }
    }

    // Double the size of the bitv
    fn grow_table(&mut self) {
        let first_number = self.bitv.len();
        // Default to true, we'll set things to false as we go
        // what's left are primes
        self.bitv.grow(first_number, true);
        let new_length = self.bitv.len();
        for i in range(1, new_length) {
            // If prime, set multiples to false
            if self.bitv.get(i) {
                // Find the first multiple of this prime in the newly created
                // table space
                let first_multiple = if i < first_number {
                    if first_number % i == 0 {
                        first_number
                    } else {
                        (first_number - first_number % i) + i
                    }
                } else {
                    i * 2
                };
                for j in range_step(first_multiple, new_length, i) {
                    self.bitv.set(j, false);
                }
            }
        }
    }
}

impl Iterator<uint> for PrimeIterator {
    fn next(&mut self) -> Option<uint> {
        loop {
            // Consider the next number
            self.current += 1;
            // If we've gone over the bounds of our Bitv, grow it
            if self.bitv.len() <= self.current {
                self.grow_table();
            }

            if self.bitv.get(self.current) {
                return Some(self.current);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{FactorIterator, is_prime, lcm, gcd, PrimeIterator};

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

    #[test]
    fn test_lowest_common_multiple() {
        assert_eq!(lcm(2, 3), 6);
    }

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(gcd(5, 5), 5);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn test_prime_iterator() {
        let mut iter = PrimeIterator::new();
        let primes = vec![
            2u, 3, 5, 7, 11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
            73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
            127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
            179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
            233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
            283, 293, 307, 311, 313, 317, 331, 337, 347, 349,
            353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
            419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
            467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
            547, 557, 563, 569, 571, 577, 587, 593, 599, 601,
            607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
            661, 673, 677, 683, 691, 701, 709, 719, 727, 733,
            739, 743, 751, 757, 761, 769, 773, 787, 797, 809,
            811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
            877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013
        ];
        for i in primes.iter() {
            println!("Looking for {}", *i);
            assert_eq!(iter.next().unwrap(), *i);
        }
    }
}
