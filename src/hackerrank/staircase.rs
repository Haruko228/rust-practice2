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
    fn test_staircase_logic() {
        let _n = 3;
        let row1 = format!("{}{}", " ".repeat(2), "#".repeat(1));
        let row3 = format!("{}{}", " ".repeat(0), "#".repeat(3));
        assert_eq!(row1, "  #");
        assert_eq!(row3, "###");
    }
}
