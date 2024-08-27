#[derive(Debug, Clone, Copy)]
pub struct HandleWrapper<T>(pub T);

unsafe impl<T> Send for HandleWrapper<T> {}

unsafe impl<T> Sync for HandleWrapper<T> {}
