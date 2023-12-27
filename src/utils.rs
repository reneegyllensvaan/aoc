pub trait Vec2dUtils<T> {
    fn transpose(&self) -> Vec<Vec<T>>;
}

impl<T: Clone> Vec2dUtils<T> for [Vec<T>] {
    /// Takes a 2d vector and transposes its axes such that columns become rows and rows become
    /// columns, moves each index (x, y) to (y, x)
    fn transpose(&self) -> Vec<Vec<T>> {
        let Some(first) = self.first() else {
            return Vec::new();
        };
        (0..first.len())
            .map(|i| {
                self.iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}
