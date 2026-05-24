// https://www.hackerrank.com/challenges/drawing-book/problem
pub fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = n / 2 - p / 2;
    from_front.min(from_back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_sample1() {
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_first_page() {
        assert_eq!(page_count(10, 0), 0);
    }

    #[test]
    fn test_last_page() {
        assert_eq!(page_count(10, 10), 0);
    }
}