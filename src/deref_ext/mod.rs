use map::Map;
use std::ops::Deref;

pub mod map;

pub trait DerefExt<A> {
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: Fn(&A) -> &B,
        B: ?Sized,
        Self: Sized;
}

impl<A, T> DerefExt<A> for T
where
    T: Deref<Target = A>,
{
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: Fn(&A) -> &B,
        B: ?Sized,
        Self: Sized,
    {
        Map::new(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::RwLock;

    #[test]
    fn test_box() {
        let s = "abc";
        let boxes = Box::new(s);
        let mapped = boxes.map(|x| x.as_bytes());
        assert_eq!(mapped.deref(), s.as_bytes());
    }

    #[test]
    fn test_lock() {
        let s = "abc";
        let lock = RwLock::new(s);
        let guard = lock.read().unwrap();
        let mapped = guard.map(|s| s.as_bytes());
        assert_eq!(mapped.deref(), s.as_bytes());
    }
}
