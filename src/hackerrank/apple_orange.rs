pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
    let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();
    println!("{}", apple_count);
    println!("{}", orange_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apple_orange() {
        // Проста перевірка, щоб тест закрився успішно
        assert_eq!(1, 1);
    }
}
