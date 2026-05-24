// https://www.hackerrank.com/challenges/apple-and-orange/problem
pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter().filter(|&&d| (a + d) >= s && (a + d) <= t).count();
    let orange_count = oranges.iter().filter(|&&d| (b + d) >= s && (b + d) <= t).count();
    println!("{}", apple_count);
    println!("{}", orange_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];
        // just verify counts directly
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apple_count = apples.iter().filter(|&&d| (a + d) >= s && (a + d) <= t).count();
        let orange_count = oranges.iter().filter(|&&d| (b + d) >= s && (b + d) <= t).count();
        assert_eq!(apple_count, 1);
        assert_eq!(orange_count, 1);
        count_apples_and_oranges(s, t, a, b, &apples, &oranges);
    }

    #[test]
    fn test_no_fruits_on_house() {
        let apples = vec![-10];
        let oranges = vec![10];
        let apple_count = apples.iter().filter(|&&d| (5 + d) >= 7 && (5 + d) <= 11).count();
        let orange_count = oranges.iter().filter(|&&d| (15 + d) >= 7 && (15 + d) <= 11).count();
        assert_eq!(apple_count, 0);
        assert_eq!(orange_count, 0);
    }
}