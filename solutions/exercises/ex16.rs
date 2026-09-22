pub fn sum_strings(left: &str, right: &str) -> Result<u64, std::num::ParseIntError> {
    let a: u32 = left.trim().parse()?;
    let b: u32 = right.trim().parse()?;
    Ok(u64::from(a) + u64::from(b))
}
