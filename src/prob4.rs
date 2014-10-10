use std::num;

// Returns true of the number is a palindrome 
fn is_palindrome(num: uint) -> bool {
    let s = num.to_string();
    let b = s.as_bytes();
    for i in range(0, b.len() / 2) {
        if b[i] != b[b.len() - i - 1] {
            return false;
        }
    }
    true
}

/// Returns the largest product between 0 and 10^(digits) that is
/// a palindrome
fn largest_palindrome_product(digits: uint) -> uint {
    let max = num::pow(10u, digits);
    let mut largest = 0u;

    for i in range(max/2, max).rev() {
        for j in range(0, max).rev() {
            let k = i * j;
            if is_palindrome(k) && k > largest {
                largest = k;
            }
        }
    }

    largest
}

#[cfg(test)]
mod test {
    use super::{is_palindrome, largest_palindrome_product};

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(9009));
        assert!(is_palindrome(90909));
        assert!(!is_palindrome(9000));
        assert!(!is_palindrome(90900));
    }

    #[test]
    fn provided_example() {
        assert_eq!(largest_palindrome_product(2), 9009);
    }

    #[test]
    fn expected_result() {
        assert_eq!(largest_palindrome_product(3), 906609);
    }
}

