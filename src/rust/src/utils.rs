use itertools::Itertools;

pub fn product<'a, T: Clone + 'a>(base: &'a [T], repeat: usize) -> Box<dyn Iterator<Item = Vec<T>> + 'a> {
    Box::new(
        std::iter::repeat(base.iter().cloned())
            .take(repeat)
            .multi_cartesian_product()
    )
}

pub fn combinations<'a, T: Clone + 'a>(pool: &'a [T], length: usize) -> Box<dyn Iterator<Item = Vec<T>> + 'a> {
    if length > pool.len() {
        return Box::new(std::iter::empty());
    }
    Box::new(pool.iter().cloned().combinations(length))
}