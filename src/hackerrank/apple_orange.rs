pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apple_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    println!("{}", apple_count);
    println!("{}", orange_count);
    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count() {
        let (apples, oranges) = count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
        assert_eq!(apples, 1);
        assert_eq!(oranges, 1);
    }
}
