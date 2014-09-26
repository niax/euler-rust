use std::iter::AdditiveIterator;
use std::num;

fn sum_square_difference(target: uint) -> uint {
    let squared_sum = range(1, target + 1).map(|a| { num::pow(a, 2) }).sum();
    let sum_squared = num::pow(range(1, target + 1).sum(), 2);
    sum_squared - squared_sum
}

#[cfg(test)]
mod test {
    use super::sum_square_difference;

    #[test]
    fn provided_example() {
        assert_eq!(sum_square_difference(10), 2640);
    }

    #[test]
    fn expected_result() {
        assert_eq!(sum_square_difference(100), 25164150);
    }
}
