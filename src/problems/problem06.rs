//! Problem 06 – Sum square difference
//!
//! Find the difference between the square of the sum 
//! and the sum of the squares for the first 100 natural numbers.
//!   (1 + 2 + … + 100)^2 – (1^2 + 2^2 + … + 100^2)

/// Solve Problem 06.
pub fn solve_06() -> u64 {
    let n = 100;
    let sum = (1..=n).sum::<u64>();
    let sum_sq = sum * sum;
    let sq_sum = (1..=n).map(|x| x * x).sum::<u64>();
    sum_sq - sq_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_10() {
        // (1+…+10)^2 = 55^2=3025; sum of squares = 385; diff=2640
        assert_eq!({
            let s = (1..=10).sum::<u64>();
            let d = s*s - (1..=10).map(|x| x*x).sum::<u64>();
            d
        }, 2640);
    }

    #[test]
    fn test_n_1() {
        assert_eq!((1..=1).sum::<u64>().pow(2) - (1..=1).map(|x|x*x).sum::<u64>(), 0);
    }

    #[test]
    fn test_n_2() {
        assert_eq!((1..=2).sum::<u64>().pow(2) - (1..=2).map(|x|x*x).sum::<u64>(), 4);
    }

    #[test]
    fn test_formula() {
        // closed‐form: diff = n(n−1)(3n+2)/6 for n=100 → 25164150
        let n = 100u64;
        let closed = n*(n-1)*(3*n+2)/6;
        assert_eq!(solve_06(), closed);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve_06(), 25164150);
    }
}
