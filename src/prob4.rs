use std::num;
use std::collections::PriorityQueue;

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

fn largest_palindrome_product(digits: uint) -> uint {
    let max = num::pow(10u, digits);
    let min = 0u;
    let mut results = PriorityQueue::new();
    
    for i in range(max/2, max).rev() {
        for j in range(0, max).rev() {
            let k = i * j;
            if is_palindrome(k) {
                results.push(k);
            }
        }
    }

    *results.top().unwrap()
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
        assert_eq!(largest_palindrome_product(2), 906609);
    }

    #[test]
    fn expected_result() {
        assert_eq!(largest_palindrome_product(3), 90909);
    }
}

