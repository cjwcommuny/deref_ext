use std::ops::Deref;

pub struct Map<T, F> {
    inner: T,
    f: F,
}

impl<T, F> Map<T, F> {
    pub(crate) fn new(inner: T, f: F) -> Self {
        Self { inner, f }
    }
}

impl<T, F, B> Deref for Map<T, F>
where
    T: Deref,
    F: Fn(&T::Target) -> &B,
    B: ?Sized,
{
    type Target = B;

    fn deref(&self) -> &Self::Target {
        (self.f)(self.inner.deref())
    }
}
