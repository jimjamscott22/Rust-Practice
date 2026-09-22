pub fn smallest<T: Ord>(values: &[T]) -> Option<&T> {
    values.iter().min()
}
