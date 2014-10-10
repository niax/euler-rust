use common::factors::prime_factors;
use common::primes::PrimeIterator;

/// Find the largest prime factor of `value`
fn largest_prime_factor(value: uint) -> uint {
    // Make a pre-seeded prime generator for prime_factors to use
    let prime_seed = PrimeIterator::new_with_size(2048);
    // Discover prime factors (as a x -> exp HashMap, where x is the prime, and exp is the number
    // of times that prime appears in the factorization)
    let prime_factors = prime_factors(value, &prime_seed);
    // Find the largest key (prime factor)
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
