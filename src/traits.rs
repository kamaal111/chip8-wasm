pub trait FillableVector<T>
where
    T: Copy,
{
    fn with_filled_capacity(size: usize, value: T) -> Vec<T>;
    fn fill(&mut self, limit: usize, value: T);
}

impl<T> FillableVector<T> for Vec<T>
where
    T: Copy,
{
    fn with_filled_capacity(size: usize, value: T) -> Vec<T> {
        let mut filled_vec: Vec<T> = Vec::with_capacity(size);
        filled_vec.fill(size, value);

        filled_vec
    }

    fn fill(&mut self, limit: usize, value: T) {
        for _ in 0..limit {
            self.push(value);
        }
    }
}
