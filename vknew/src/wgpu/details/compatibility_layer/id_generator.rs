pub struct IdGenerator<T>
where
    T: From<u64>,
{
    counter: u64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> IdGenerator<T>
where
    T: From<u64>,
{
    pub fn new() -> Self {
        Self {
            counter: 1,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn generate(&mut self) -> T {
        let id = self.counter;
        self.counter += 1;
        T::from(id)
    }
}
