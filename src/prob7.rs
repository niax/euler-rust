use common::PrimeIterator;

/// Returns the nth prime.
fn nth_prime(n: uint) -> uint {
    let mut prime_iter = PrimeIterator::new();
    prime_iter.nth(n - 1).unwrap()
}

#[cfg(test)]
mod test {
    use super::nth_prime;


    #[test]
    fn provided_example() {
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn expected_result() {
       assert_eq!(nth_prime(10_001), 104743);
    }
}
