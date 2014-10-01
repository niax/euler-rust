use common::factors::prime_factors;
use common::primes::PrimeIterator;

fn largest_prime_factor(value: uint) -> uint {
    let prime_seed = PrimeIterator::new_with_size(2048);
    let prime_factors = prime_factors(value, &prime_seed);
    *(prime_factors.keys().max_by(|x| { *x }).unwrap())
}

#[cfg(test)]
mod test {
    extern crate test;

    use self::test::Bencher;
    use super::largest_prime_factor;

    #[test]
    fn provided_example() {
        assert_eq!(largest_prime_factor(13195), 29u)
    }

    #[test]
    fn expected_result() {
        assert_eq!(largest_prime_factor(600851475143), 6857u)
    }

    #[bench]
    fn bench_largest(b: &mut Bencher) {
        b.iter(|| {
            largest_prime_factor(600851475143)
        });
    }
}
