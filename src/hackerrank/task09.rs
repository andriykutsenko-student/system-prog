// https://www.hackerrank.com/challenges/migratory-birds/problem
pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0i32; 6];
    for &b in arr {
        counts[b as usize] += 1;
    }
    let max = *counts[1..].iter().max().unwrap();
    (1..=5).find(|&i| counts[i] == max).unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
    }

    #[test]
    fn test_sample1() {
        assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
    }

    #[test]
    fn test_tie_lowest_wins() {
        assert_eq!(migratory_birds(&[1, 2, 1, 2]), 1);
    }
}