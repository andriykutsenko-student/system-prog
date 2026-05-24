// https://www.hackerrank.com/challenges/between-two-sets/problem
pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    (max_a..=min_b)
        .filter(|&x| {
            a.iter().all(|&ai| x % ai == 0) && b.iter().all(|&bi| bi % x == 0)
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }

    #[test]
    fn test_single_elements() {
        assert_eq!(get_total_x(&[1], &[72]), 12);
    }

    #[test]
    fn test_no_between() {
        assert_eq!(get_total_x(&[3], &[10]), 0);
    }
}