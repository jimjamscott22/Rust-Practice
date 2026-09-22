pub fn parse_count(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.trim().parse()
}
