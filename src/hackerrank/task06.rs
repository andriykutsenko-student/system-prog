// https://www.hackerrank.com/challenges/kangaroo/problem
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 == v2 {
        if x1 == x2 { "YES" } else { "NO" }
    } else if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) > 0 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_sample1() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_same_position() {
        assert_eq!(kangaroo(5, 3, 5, 3), "YES");
    }

    #[test]
    fn test_same_speed_different_pos() {
        assert_eq!(kangaroo(0, 3, 5, 3), "NO");
    }
}