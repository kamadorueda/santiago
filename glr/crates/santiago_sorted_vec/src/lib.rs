pub trait SortedVec<T> {
    fn sorted_contains(&self, data: &T) -> bool
    where
        Self: AsRef<[T]>,
        T: Ord,
    {
        self.as_ref().binary_search(data).is_ok()
    }

    fn sorted_insert(&mut self, data: T) -> usize
    where
        Self: AsMut<Vec<T>>,
        T: Ord,
    {
        let s = self.as_mut();

        match s.binary_search(&data) {
            Ok(index) => index,
            Err(index) => {
                s.insert(index, data);
                index
            },
        }
    }
}

impl<T> SortedVec<T> for Vec<T> where T: Ord {}
