use std::iter::AdditiveIterator;

struct MultipleIterator {
    current: uint,
    max: uint,
}

impl MultipleIterator {
    fn new(start: uint, max: uint) -> MultipleIterator {
        MultipleIterator {
            current: start,
            max: max
        }
    }
}

impl Iterator<uint> for MultipleIterator {
    fn next(&mut self) -> Option<uint> {
        while self.current < self.max {
            if self.current % 3 == 0 || self.current % 5 == 0 {
                self.current += 1;
                return Some(self.current - 1)
            }
            self.current += 1;
        }
        None
    }
}


/// Returns the sum of multiples of 3 or 5 between 0 and the `target`
fn sum_multiples(target: uint) -> uint{
    let mut iter = MultipleIterator::new(0, target);
    iter.sum()
}

#[cfg(test)]
mod test {
    use super::{MultipleIterator,sum_multiples};

    #[test]
    fn test_iterator() {
        let mut result = Vec::new();
        let mut iter = MultipleIterator::new(0, 10);
        for i in iter {
            result.push(i);
        }
        assert_eq!(result, vec![0, 3, 5, 6, 9]);
    }


    #[test]
    fn provided_example() {
        assert_eq!(sum_multiples(10), 23u);
    }

    #[test]
    fn expected_result() {
        assert_eq!(sum_multiples(1000), 233168u);
    }
}
