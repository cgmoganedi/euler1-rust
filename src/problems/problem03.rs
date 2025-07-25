//! Problem 03 – Largest Prime Factor
//!
//! Find the largest prime factor of the number 600351475143.

/// Solve Problem 03.
pub fn solve_03() -> u64 {
    let mut n = 600_851_475_143u64;
    let mut max_factor = 0;

    // Remove factors of 2
    while n % 2 == 0 {
        max_factor = 2;
        n /= 2;
    }

    // Check odd factors up to sqrt(n)
    let mut f = 3;
    while f * f <= n {
        while n % f == 0 {
            max_factor = f;
            n /= f;
        }
        f += 2;
    }

    // If remainder > 2, it's prime
    if n > 2 {
        max_factor = n;
    }
    max_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_number() {
        assert_eq!({
            let n = 13195;
            let mut max = 0;
            // same factor logic on 13195 → prime factors 5, 7, 13, 29
            let mut n = n;
            while n % 2 == 0 { max = 2; n /= 2; }
            let mut f = 3;
            while f * f <= n {
                while n % f == 0 {
                    max = f; n /= f;
                }
                f += 2;
            }
            if n > 2 { max = n; }
            max
        }, 29);
    }

    #[test]
    fn test_prime_input() {
        assert_eq!({
            let mut n = 17;
            let mut max = 0;
            while n % 2 == 0 { max = 2; n /= 2; }
            let mut f = 3;
            while f * f <= n {
                while n % f == 0 { max = f; n /= f; }
                f += 2;
            }
            if n > 2 { max = n; }
            max
        }, 17);
    }

    #[test]
    fn test_two_factors() {
        assert_eq!({
            let mut n = 35;
            let mut max = 0;
            while n % 2 == 0 { max = 2; n /= 2; }
            let mut f = 3;
            while f * f <= n {
                while n % f == 0 { max = f; n /= f; }
                f += 2;
            }
            if n > 2 { max = n; }
            max
        }, 7);
    }

    #[test]
    fn test_even_only() {
        // 2^6 = 64 → largest prime factor is 2
        let mut n = 64;
        let mut max = 0;
        while n % 2 == 0 { max = 2; n /= 2; }
        assert_eq!(max, 2);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve_03(), 6857);
    }
}
