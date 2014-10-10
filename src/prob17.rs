static BASE_NUMBERS: [&'static str, .. 21] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty"
];

static TENS: [&'static str, .. 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];

/// Returns the given `n` as a string of words
/// 
/// *NOTE* - `n` must be less than 10000 and not negative.
fn number_as_words(n: uint) -> String {
    match n {
        0...20 => BASE_NUMBERS[n].to_string(),
        20...99 => {
            if n % 10 == 0 {
                TENS[n/10].to_string()
            } else {
                format!("{}-{}", TENS[n/10], BASE_NUMBERS[n % 10])
            }
        },
        100...999  => {
            if n % 100 == 0 {
                format!("{} hundred", BASE_NUMBERS[n/100])
            } else {
                format!("{} hundred and {}", BASE_NUMBERS[n/100], number_as_words(n % 100))
            }
        },
        1000...9999  => {
            if n % 1000 == 0 {
                format!("{} thousand", BASE_NUMBERS[n/1000])
            } else {
                format!("{} thousand {}", BASE_NUMBERS[n/1000], number_as_words(n % 1000))
            }
        },
        _ => fail!("Don't know this number")
    }
}

/// Returns the number of characters (exclusive of spaces and hyphens) used when
/// writing out the numbers one to `max`.
fn sum_char_count(max: uint) -> uint {
    let mut sum = 0u;
    for i in range(1, max + 1) {
        let words = number_as_words(i);
        // Spaces and hyphens don't count for our character count
        sum += words.replace(" ", "").replace("-", "").len();
    }
    sum
}

#[cfg(test)]
mod test {
    use super::{number_as_words, sum_char_count};

    #[test]
    fn test_number_as_words() {
        assert_eq!(number_as_words(1), "one".to_string());
        assert_eq!(number_as_words(13), "thirteen".to_string());
        assert_eq!(number_as_words(42), "forty-two".to_string());
        assert_eq!(number_as_words(137), "one hundred and thirty-seven".to_string());
        assert_eq!(number_as_words(200), "two hundred".to_string());
        assert_eq!(number_as_words(1337), "one thousand three hundred and thirty-seven".to_string());
        assert_eq!(number_as_words(2000), "two thousand".to_string());
    }

    #[test]
    fn given_example() {
        assert_eq!(sum_char_count(5), 19);
    }

    #[test]
    fn expected_result() {
        assert_eq!(sum_char_count(1000), 21124);
    }
}
