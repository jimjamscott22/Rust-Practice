pub fn unique(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in values {
        if !result.contains(&n) {
            result.push(n);
        }
    }
    result
}
