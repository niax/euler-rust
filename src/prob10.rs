use common::PrimeIterator;

fn sum_of_primes(less_than: uint) -> uint {
    let mut sum = 0u;
    let mut iter = PrimeIterator::new();
    for i in iter {
        if i >= less_than {
            break;
        }
        sum += i;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::sum_of_primes;


    #[test]
    fn provided_example() {
        assert_eq!(sum_of_primes(10), 17u);
    }

    #[test]
    fn expected_result() {
       assert_eq!(sum_of_primes(2_000_000), 142913828922u);
    }
}
