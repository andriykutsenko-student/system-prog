// https://www.hackerrank.com/challenges/divisible-sum-pairs/problem
pub fn divisible_sum_pairs(ar: &[i32], k: i32) -> i32 {
    let n = ar.len();
    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(divisible_sum_pairs(&[1, 3, 2, 6, 1, 2], 3), 5);
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(divisible_sum_pairs(&[1, 2, 3], 7), 0);
    }

    #[test]
    fn test_all_pairs() {
        assert_eq!(divisible_sum_pairs(&[3, 3, 3], 3), 3);
    }
}