// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut max = scores[0];
    let mut min = scores[0];
    let mut max_count = 0;
    let mut min_count = 0;

    for &s in &scores[1..] {
        if s > max {
            max = s;
            max_count += 1;
        } else if s < min {
            min = s;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        assert_eq!(breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1]), vec![2, 4]);
    }

    #[test]
    fn test_sample1() {
        assert_eq!(breaking_records(&[3, 4, 21, 36, 10, 28, 35, 5, 24, 42]), vec![4, 0]);
    }

    #[test]
    fn test_single_game() {
        assert_eq!(breaking_records(&[5]), vec![0, 0]);
    }
}