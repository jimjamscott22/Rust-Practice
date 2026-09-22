pub fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.chars().count() >= right.chars().count() {
        left
    } else {
        right
    }
}
