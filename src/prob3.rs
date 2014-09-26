use common::{FactorIterator,is_prime};

fn largest_prime_factor(value: uint) -> Option<uint> {
    let max = (value as f64).sqrt().floor();
    let iter = FactorIterator::new(value, max as uint);
    let mut filtered = iter.filter(|x| { is_prime(*x) });
    filtered.max_by(|x| { *x })
}

#[cfg(test)]
mod test {
    use super::largest_prime_factor;

    #[test]
    fn provided_example() {
        assert_eq!(largest_prime_factor(13195).unwrap(), 29u)
    }

    #[test]
    fn expected_result() {
        assert_eq!(largest_prime_factor(600851475143).unwrap(), 6857u)
    }
}
