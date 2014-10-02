extern crate bignum;

use std::num::pow;
use self::bignum::{ToBigUint, BigUint};

fn sum_of_digits(n: BigUint) -> BigUint {
    let zero = 0u.to_biguint().unwrap();
    let ten = 10u.to_biguint().unwrap();
    let mut remaining = n;
    let mut sum = zero.clone();
    while remaining > zero {
        sum = sum + (remaining % ten);
        remaining = remaining / ten;
    }
    sum
}

fn sum_of_power_two(n: uint) -> BigUint {
    let pow2 = pow(2u.to_biguint().unwrap(), n);
    sum_of_digits(pow2)
}

#[cfg(test)]
mod test {
    extern crate bignum;

    use super::sum_of_power_two;
    use self::bignum::ToBigUint;

    #[test]
    fn given_example() {
        assert_eq!(sum_of_power_two(15), 26u.to_biguint().unwrap());
    }

    #[test]
    fn expected_result() {
        assert_eq!(sum_of_power_two(1000), 1366u.to_biguint().unwrap());
    }
}
