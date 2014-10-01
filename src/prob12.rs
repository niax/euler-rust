use common::primes::{PrimeIterator, is_prime};
use std::collections::HashMap;


fn prime_factors_recurse(n: uint, prime_seed: &PrimeIterator, factors: &mut HashMap<uint, uint>) {
    let mut prime_iter = PrimeIterator::new_from_seed(prime_seed);
    let new_factor = if is_prime(n) {
        n
    } else {
        let mut fact = 0u;
        for i in prime_iter {
            if n % i == 0 {
                prime_factors_recurse(n / i, prime_seed, factors);
                fact = i;
                break;
            }
        }
        fact
    };

    let new_count = match factors.find(&new_factor) {
        Some(old_count) => old_count + 1,
        None => 1
    };
    factors.insert(new_factor, new_count);
}

fn prime_factors(n: uint, prime_seed: &PrimeIterator) -> HashMap<uint, uint> {
    let mut factors: HashMap<uint, uint> = HashMap::new();

    prime_factors_recurse(n, prime_seed, &mut factors);

    factors
}

fn p(divisor_count: uint) -> uint {
    let prime_seed = PrimeIterator::new_with_size(8192);
    let mut i = 2u;
    loop {
        let triangle_number = (i * (i + 1)) / 2;
        let factors = prime_factors(triangle_number, &prime_seed);
        // Multiply together all the "exponents" of the prime factors (with one added on)
        let factor_count = factors.values().map(|&x| { x + 1 }).fold(1u, |a, b| { a * b });
        if factor_count > divisor_count {
            return triangle_number;
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::{p, prime_factors};
    use common::primes::PrimeIterator;
    use std::collections::HashMap;

    #[test]
    fn test_prime_factors() {
        let prime_seed = PrimeIterator::new_with_size(1024);
        let factors = prime_factors(864, &prime_seed);
        let mut expected = HashMap::new();
        expected.insert(2u, 5u);
        expected.insert(3u, 3u);
        assert_eq!(factors, expected);
    }

    #[test]
    fn provided_example() {
        assert_eq!(p(5), 28u);
    }

    #[test]
    fn expected_result() {
        assert_eq!(p(500), 76576500u);
    }
}
