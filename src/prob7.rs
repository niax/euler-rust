struct PrimeIterator {
    previous: Vec<uint>,
    current: uint,
}


impl PrimeIterator {
    pub fn new() -> PrimeIterator {
        PrimeIterator {
            previous: Vec::new(),
            current: 1
        }
    }
}

impl Iterator<uint> for PrimeIterator {
    fn next(&mut self) -> Option<uint> {
        if self.current == 1 {
            self.current = 2;
            self.previous.push(2);
            println!("{}", self.current);
            return Some(2);
        }

        loop {
            self.current += 1;
            let mut good = true;
            // limit division testing by the largest possible factor
            let max_factor = (self.current as f64).sqrt().ceil() as uint;
            for &i in self.previous.iter() {
                if self.current % i == 0 && i <= max_factor {
                    good = false;
                    break;
                }
            }

            if good {
                self.previous.push(self.current);
                println!("{}", self.current);
                return Some(self.current);
            }
        }
    }
}

/// Returns the nth prime.
fn nth_prime(n: uint) -> uint {
    let mut prime_iter = PrimeIterator::new();
    prime_iter.nth(n - 1).unwrap()
}

#[cfg(test)]
mod test {
    use super::{nth_prime, PrimeIterator};

    #[test]
    fn test_prime_iterator() {
        let mut iter = PrimeIterator::new();
        assert_eq!(iter.next().unwrap(), 2u);
        assert_eq!(iter.next().unwrap(), 3u);
        assert_eq!(iter.next().unwrap(), 5u);
        assert_eq!(iter.next().unwrap(), 7u);
        assert_eq!(iter.next().unwrap(), 11u);
        assert_eq!(iter.next().unwrap(), 13u);
    }

    #[test]
    fn provided_example() {
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn expected_result() {
       assert_eq!(nth_prime(10_001), 104743);
    }
}
