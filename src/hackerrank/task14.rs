// https://www.hackerrank.com/challenges/bon-appetit/problem
pub fn bon_appetit(bill: &[i32], k: usize, b: i32) -> String {
    let actual: i32 = bill.iter().enumerate()
        .filter(|&(i, _)| i != k)
        .map(|(_, &x)| x)
        .sum::<i32>() / 2;
    if b == actual {
        "Bon Appetit".to_string()
    } else {
        (b - actual).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overcharged() {
        assert_eq!(bon_appetit(&[3, 10, 2, 9], 1, 12), "5");
    }

    #[test]
    fn test_correct() {
        assert_eq!(bon_appetit(&[3, 10, 2, 9], 1, 7), "Bon Appetit");
    }

    #[test]
    fn test_first_item_skipped() {
        assert_eq!(bon_appetit(&[10, 2, 4], 0, 3), "Bon Appetit");
    }
}