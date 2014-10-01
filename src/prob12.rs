use common::primes::PrimeIterator;
use common::factors::prime_factors;


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
    use super::p;

    #[test]
    fn provided_example() {
        assert_eq!(p(5), 28u);
    }

    #[test]
    fn expected_result() {
        assert_eq!(p(500), 76576500u);
    }
}
