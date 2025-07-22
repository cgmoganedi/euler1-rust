fn sum_multiples(limit: usize) -> usize {
    (1..limit)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn main() {
    let limit = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    println!("{}", sum_multiples(limit));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(sum_multiples(10), 23);
    }

    #[test]
    fn test_zero() {
        assert_eq!(sum_multiples(0), 0);
    }
}