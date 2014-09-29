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

#[cfg(test)]
mod test {
    use super::{FactorIterator, is_prime, lcm, gcd};

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
}
