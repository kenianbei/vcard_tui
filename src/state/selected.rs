#[derive(Clone, PartialEq, Eq)]
pub struct Selected<T> {
    inner: T,
}

impl<T: PartialEq> Selected<T> {
    pub fn get(&self) -> &T {
        &self.inner
    }
    pub fn set(&mut self, inner: T) {
        self.inner = inner
    }
    pub fn is(&self, inner: T) -> bool {
        self.inner == inner
    }
}

pub trait Select {
    fn prev(&mut self);
    fn next(&mut self);
}

impl<T> From<T> for Selected<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}
