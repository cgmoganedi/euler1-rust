//! Problem 01 – Multiples of 3 and 5
//!
//! Sum all natural numbers below 1000 that are divisible by 3 or 5.

/// Solve Problem 01.
pub fn solve_01() -> u64 {
    (1..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: generic sum of multiples below a given limit.
    fn sum_multiples(limit: u64) -> u64 {
        (1..limit).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
    }

    #[test]
    fn test_limit_10() {
        assert_eq!(sum_multiples(10), 23);
    }

    #[test]
    fn test_limit_15() {
        assert_eq!(sum_multiples(15), 60);
    }

    #[test]
    fn test_limit_1() {
        assert_eq!(sum_multiples(1), 0);
    }

    #[test]
    fn test_limit_0() {
        assert_eq!(sum_multiples(0), 0);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve_01(), 233168);
    }
}
