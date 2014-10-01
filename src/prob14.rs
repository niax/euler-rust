struct CollatzIterator {
    current: uint,
    first: bool,
}

impl CollatzIterator {
    pub fn new(starting: uint) -> CollatzIterator {
        CollatzIterator {
            current: starting,
            first: true,
        }
    }
}

impl Iterator<uint> for CollatzIterator {
    fn next(&mut self) -> Option<uint> {
        if self.current == 1 {
            None
        } else if self.first {
            // Handle first number
            self.first = false;
            Some(self.current)
        }else {
            self.current = if self.current % 2 == 0 {
                // n is even -> n/2
                self.current / 2
            } else {
                // n is odd -> 3n + 1
                self.current * 3 + 1
            };
            Some(self.current)
        }
    }
}

fn find_longest_chain(less_than: uint) -> uint {
    let mut longest_length = 1u;
    let mut longest_starting = 1u;
    for i in range(2, less_than) {
        let mut iter = CollatzIterator::new(i);
        let length = iter.count();
        if length > longest_length {
            longest_length = length;
            longest_starting = i;
        }
    }
    longest_starting
}

#[cfg(test)]
mod test {
    use super::{CollatzIterator, find_longest_chain};

    #[test]
    fn test_collatz() {
        let mut iter = CollatzIterator::new(13);
        let known_values = vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1];
        let given_values: Vec<uint> = iter.collect();
        assert_eq!(given_values, known_values);
    }

    #[test]
    fn expected_result() {
        assert_eq!(find_longest_chain(1_000_000), 837799);
    }

}
