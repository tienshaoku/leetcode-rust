pub fn normalise(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for inner in v.iter_mut() {
        inner.sort();
    }
    v.sort();
    v
}
