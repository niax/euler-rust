use std::iter::range_step;
use std::collections::Bitv;

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

    /// Create a new prime iterator starting at 2 with the internal
    /// data structures already calculated up to at least size
    pub fn new_with_size(size: uint) -> PrimeIterator {
        let mut iter = PrimeIterator::new();
        while iter.bitv.len() < size {
            iter.grow_table();
        }
        iter
    }

    /// Create a new prime iterator starting at 2 based off the 
    /// calculations already performed by a previous iterator
    pub fn new_from_seed(seed: &PrimeIterator) -> PrimeIterator {
        PrimeIterator {
            current: 0,
            bitv: seed.bitv.clone(),
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
    use super::{is_prime, PrimeIterator};

    #[test]
    fn test_primality() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
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
            assert_eq!(iter.next().unwrap(), *i);
        }
    }
}
