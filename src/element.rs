pub struct Element<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Element<T> {
    pub fn none() -> Self {
        Element {
            _phantom: std::marker::PhantomData,
        }
    }
}
