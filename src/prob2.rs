use std::iter::AdditiveIterator;

/// Iterator over the Fibonacci series, limited by `max`
struct FibonacciIterator {
    last: uint,
    penultimate: uint,
    max: uint,
}

impl FibonacciIterator {
    fn new(max: uint) -> FibonacciIterator {
        FibonacciIterator {
            last: 1u,
            penultimate: 0u,
            max: max,
        }
    }
}

impl Iterator<uint> for FibonacciIterator {
    fn next(&mut self) -> Option<uint> {
        let fib = self.last + self.penultimate;
        self.penultimate = self.last;
        self.last = fib;
        if fib <= self.max {
            Some(fib)
        } else {
            None
        }
    }
}
/// Returns the sum of the even fibonacci numbers less than `target`
fn sum_fibonacci(target: uint) -> uint{
    // filter the fibonacci numbers so only even numbers remain in the iterator
    let mut iter = FibonacciIterator::new(target).filter(|a| { *a % 2 == 0 });
    iter.sum()
}

#[cfg(test)]
mod test {
    use super::{FibonacciIterator,sum_fibonacci};

    #[test]
    fn test_iterator() {
        let mut result = Vec::new();
        let mut iter = FibonacciIterator::new(10);
        for i in iter {
            result.push(i);
        }
        assert_eq!(result, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn provided_example() {
        assert_eq!(sum_fibonacci(89), 44u);
    }

    #[test]
    fn expected_result() {
        assert_eq!(sum_fibonacci(4_000_000), 4613732u);
    }
}
