use std::num::{one, One};

/// Calculates the greatest common divisor of `a` and `b`
pub fn gcd(a: uint, b: uint) -> uint {
    let mut i = a;
    let mut j = b;
    while i != j {
        if i > j {
            i = i - j;
        } else {
            j = j - i;
        }
    }
    i
}

/// Calculates the lowest common multiple of `a` and `b`
pub fn lcm(a: uint, b: uint) -> uint {
    if a == 0 && b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

/// Trait for number types that implement Factorial
pub trait Factorial {
    /// Calculate a factorial n!
    fn factorial(&self) -> Self;
}

impl<T: One + PartialOrd + Clone + Add<T, T> + Mul<T, T> + ToPrimitive> Factorial for T {
    fn factorial(&self) -> T {
        range(one::<T>(), *self + one::<T>()).fold(one::<T>(), |a, b| { a * b })
    }
}

#[cfg(test)]
mod test {
    use super::{lcm, gcd, Factorial};

    #[test]
    fn test_lowest_common_multiple() {
        assert_eq!(lcm(2, 3), 6);
    }

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(gcd(5, 5), 5);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(5u.factorial(), 120u);
        assert_eq!(5i.factorial(), 120i);
        assert_eq!((5.0 as f64).factorial(), 120.0);
    }
}
