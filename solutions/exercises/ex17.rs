pub fn even_squares(values: &[i32]) -> Vec<i64> {
    values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| i64::from(n) * i64::from(n))
        .collect()
}
