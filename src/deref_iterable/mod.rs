use std::ops::Deref;

pub struct DerefIterable<T> {
    inner: T,
}

impl<T> DerefIterable<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<'a, T, Iter> IntoIterator for &'a DerefIterable<T>
where
    T: Deref,
    &'a T::Target: IntoIterator<Item = Iter::Item, IntoIter = Iter>,
    Iter: Iterator,
{
    type Item = Iter::Item;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.deref().into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::deref_iterable::DerefIterable;
    use std::sync::RwLock;

    #[test]
    fn test() {
        let v = vec!["hello".to_string(), "world".to_string()];
        let lock = RwLock::new(v.clone());
        let guard = lock.read().unwrap();
        let deref = DerefIterable::new(guard);
        let result = use_iter(&deref);
        assert_eq!(v, result);
    }

    fn use_iter<'a>(iter: impl IntoIterator<Item = &'a String>) -> Vec<String> {
        iter.into_iter().map(ToString::to_string).collect()
    }
}
