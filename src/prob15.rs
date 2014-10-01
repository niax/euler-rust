use common::math::Factorial;

fn path_count(n: uint) -> uint {
    // Make n a float because uint won't hold large enough numbers
    let float_n = n as f64;
    let fac_n = float_n.factorial();
    // Number of paths is (2n)!/(n! * n!) because of binomial expansion
    let result = (2.0 * float_n).factorial() / (fac_n * fac_n);
    result as uint
}


#[cfg(test)]
mod test {
    use super::path_count;

    #[test]
    fn provided_example() {
        assert_eq!(path_count(2), 6);
    }

    #[test]
    fn expected_result() {
        assert_eq!(path_count(20), 137846528820);
    }
}
