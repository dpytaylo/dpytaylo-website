pub trait ResourcePointer<T> {
    fn create(value: T) -> Self;
}