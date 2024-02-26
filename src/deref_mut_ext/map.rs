use std::ops::{Deref, DerefMut};

pub struct Map<T, F1, F2> {
    inner: T,
    f1: F1,
    f2: F2,
}

impl<T, F1, F2> Map<T, F1, F2> {
    pub(crate) fn new(inner: T, f1: F1, f2: F2) -> Self {
        Self { inner, f1, f2 }
    }
}

impl<T, F1, F2, B> Deref for Map<T, F1, F2>
where
    B: ?Sized,
    F1: Fn(&T::Target) -> &B,
    F2: Fn(&mut T::Target) -> &mut B,
    T: DerefMut,
{
    type Target = B;

    fn deref(&self) -> &Self::Target {
        (self.f1)(self.inner.deref())
    }
}

impl<T, F1, F2, B> DerefMut for Map<T, F1, F2>
where
    T: DerefMut,
    F1: Fn(&T::Target) -> &B,
    F2: Fn(&mut T::Target) -> &mut B,
    B: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        (self.f2)(self.inner.deref_mut())
    }
}
