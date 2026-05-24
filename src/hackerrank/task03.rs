pub fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_staircase_n6() {
        let result = build_staircase(6);
        let expected = vec![
            "     #",
            "    ##",
            "   ###",
            "  ####",
            " #####",
            "######",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_staircase_n1() {
        assert_eq!(build_staircase(1), vec!["#"]);
    }

    #[test]
    fn test_staircase_n3() {
        assert_eq!(build_staircase(3), vec!["  #", " ##", "###"]);
    }

    fn build_staircase(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| format!("{}{}", " ".repeat((n - i) as usize), "#".repeat(i as usize)))
            .collect()
    }
}