pub fn sum_to(n: u32) -> u64 {
    let mut total = 0u64;
    for value in 1..=n {
        total += u64::from(value);
    }
    total
}
