// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashMap;

pub fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &s in ar {
        *counts.entry(s).or_insert(0) += 1;
    }
    counts.values().map(|&c| c / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(sock_merchant(&[10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
    }

    #[test]
    fn test_all_pairs() {
        assert_eq!(sock_merchant(&[1, 1, 2, 2]), 2);
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(sock_merchant(&[1, 2, 3]), 0);
    }
}