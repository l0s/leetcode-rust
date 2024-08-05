pub fn array_to_vec<T, S>(grid: &[T]) -> Vec<Vec<S>>
where
    T: AsRef<[S]>,
    S: Clone,
{
    grid.iter().map(|array| array.as_ref().to_vec()).collect()
}
