//! Problem 01 - Multiples of 3 and 5
//!
//! Sum all natural numbers below 1000 that are divisible by 3 or 5.

pub fn solve_01() -> usize {
    (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_limit_10() {
        assert_eq!((1..10).filter(|n| n % 3 == 0 || n % 5 == 0).sum::<usize>(), 23);
    }

    #[test]
    fn problem_output() {
        assert_eq!(solve_01(), 233168);
    }

    #[test]
    fn limit_15_check() {
        assert_eq!((1..15).filter(|n| n % 3 == 0 || n % 5 == 0).sum::<usize>(), 60);
    }

    #[test]
    fn limit_1_should_be_0() {
        assert_eq!((1..1).filter(|n| n % 3 == 0 || n % 5 == 0).sum::<usize>(), 0);
    }

    #[test]
    fn edge_case_0() {
        assert_eq!((1..0).filter(|n| n % 3 == 0 || n % 5 == 0).sum::<usize>(), 0);
    }
}
