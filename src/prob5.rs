use common::math::lcm;

/// Returns the smallest number than can divided by all numbers between one and `target`
fn smallest_multiple(target: uint) -> uint {
    range(1, target).fold(1, |a, b| { lcm(a, b) })
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::smallest_multiple;
    use self::test::Bencher;

    #[test]
    fn provided_example() {
        assert_eq!(smallest_multiple(10), 2520);
    }

    #[test]
    fn expected_result() {
        assert_eq!(smallest_multiple(20), 232792560);
    }

    #[bench]
    fn bench_smallest_multiple(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(smallest_multiple(20), 232792560);
        });
    }
}
