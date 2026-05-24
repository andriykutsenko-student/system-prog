// https://www.hackerrank.com/challenges/birthday-cake-candles/problem
pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max = candles.iter().max().unwrap();
    candles.iter().filter(|&c| c == max).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        assert_eq!(birthday_cake_candles(&[3, 2, 1, 3]), 2);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(birthday_cake_candles(&[5, 5, 5]), 3);
    }

    #[test]
    fn test_one_tallest() {
        assert_eq!(birthday_cake_candles(&[1, 2, 3]), 1);
    }
}