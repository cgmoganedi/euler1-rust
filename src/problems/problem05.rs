//! Problem 05 – Smallest Multiple
//!
//! What is the smallest positive number evenly divisible by all of 1..=20?

/// Compute the greatest common divisor (Euclid’s algorithm).
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Compute least common multiple via a * b / gcd(a, b).
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// Solve Problem 05.
pub fn solve_05() -> u64 {
    (1..=20).fold(1, |acc, x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(14, 21), 7);
    }

    #[test]
    fn test_lcm_basic() {
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn test_lcm_sequence() {
        // LCM of 1..=10 = 2520
        let ans = (1..=10).fold(1, |acc, x| lcm(acc, x));
        assert_eq!(ans, 2520);
    }

    #[test]
    fn test_partial_fold() {
        // Check LCM of [3,4,5] = 60
        assert_eq!( (1..=5).fold(1, |a,x| if x<6 { lcm(a,x) } else { a }), 60 );
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve_05(), 232792560);
    }
}
