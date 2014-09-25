
/// Returns the sum of multiples of 3 or 5 between 0 and the `target`
fn sum_multiples(target: uint) -> uint{
    let mut sum = 0u;
    for i in range(0, target) {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
            println!("{} - {}", i, sum);
                
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::sum_multiples;

    #[test]
    fn provided_example() {
        assert_eq!(sum_multiples(10), 23u);
    }

    #[test]
    fn expected_result() {
        assert_eq!(sum_multiples(1000), 233168u);
    }
}
