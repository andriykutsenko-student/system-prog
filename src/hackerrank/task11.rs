// https://www.hackerrank.com/challenges/diagonal-difference/problem
pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let primary: i32 = (0..n).map(|i| arr[i][i]).sum();
    let secondary: i32 = (0..n).map(|i| arr[i][n - 1 - i]).sum();
    (primary - secondary).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(&arr), 15);
    }

    #[test]
    fn test_1x1() {
        assert_eq!(diagonal_difference(&[vec![5]]), 0);
    }

    #[test]
    fn test_equal_diagonals() {
        let arr = vec![
            vec![1, 2],
            vec![2, 1],
        ];
        assert_eq!(diagonal_difference(&arr), 2);
    }
}