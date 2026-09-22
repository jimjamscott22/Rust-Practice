pub fn first_even(values: &[i32]) -> Option<i32> {
    for &n in values {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}
