pub fn normalise<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    v.sort();
    v
}
