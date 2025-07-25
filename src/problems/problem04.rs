//! Problem 04 – Largest Palindrome Product
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

/// Check if a number is a palindrome in base 10.
fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}

/// Solve Problem 04.
pub fn solve_04() -> u64 {
    let mut max_pal = 0;
    for i in 100..=999 {
        for j in i..=999 {
            let prod = i * j;
            if prod > max_pal && is_palindrome(prod) {
                max_pal = prod;
            }
        }
    }
    max_pal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_true() {
        assert!(is_palindrome(9009)); // 91*99
    }

    #[test]
    fn test_is_palindrome_false() {
        assert!(!is_palindrome(1234));
    }

    #[test]
    fn test_small_range() {
        // for 2-digit numbers (10..99), largest palindrome is 9009? No,
        // actually 91*99 = 9009 > max for 2-digits, so test 2-digit
        let mut max = 0;
        for i in 10..=99 {
            for j in i..=99 {
                let p = i * j;
                if p > max && {
                    let s = p.to_string();
                    s.chars().eq(s.chars().rev())
                } {
                    max = p;
                }
            }
        }
        assert_eq!(max, 9009);
    }

    #[test]
    fn test_3digit_min() {
        // verify that we find at least a known palindrome
        assert!(solve_04() >= 9009);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve_04(), 906609);
    }
}
