//! Problem 02 – Even Fibonacci Numbers
//!
//! Sum all even Fibonacci numbers not exceeding 4,000,000.

/// Solve Problem 02.
pub fn solve_02() -> u64 {
    let (mut a, mut b) = (1, 2);
    let mut sum = 0;
    while b <= 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compute sum of even‐valued Fibonacci terms ≤ limit.
    fn sum_even_fibs(limit: u64) -> u64 {
        let (mut a, mut b) = (1, 2);
        let mut sum = 0;
        while b <= limit {
            if b % 2 == 0 { sum += b; }
            let next = a + b;
            a = b;
            b = next;
        }
        sum
    }

    #[test]
    fn test_limit_10() {
        assert_eq!(sum_even_fibs(10), 10);
    }

    #[test]
    fn test_limit_100() {
        assert_eq!(sum_even_fibs(100), 44);
    }

    #[test]
    fn test_limit_1() {
        assert_eq!(sum_even_fibs(1), 0);
    }

    #[test]
    fn test_limit_2() {
        assert_eq!(sum_even_fibs(2), 2);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve_02(), 4_613_732);
    }
}
