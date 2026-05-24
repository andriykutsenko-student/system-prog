// https://www.hackerrank.com/challenges/grading/problem
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&g| round_grade(g)).collect()
}

fn round_grade(g: i32) -> i32 {
    if g < 38 {
        return g;
    }
    let next_multiple = (g / 5 + 1) * 5;
    if next_multiple - g < 3 {
        next_multiple
    } else {
        g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_no_round_below_38() {
        assert_eq!(grading_students(&[33]), vec![33]);
    }

    #[test]
    fn test_exact_multiple() {
        assert_eq!(grading_students(&[40]), vec![40]);
    }
}