mod map;

use crate::deref_mut_ext::map::Map;
use std::ops::DerefMut;

pub trait DerefMutExt<A> {
    fn bimap<B, F1, F2>(self, f1: F1, f2: F2) -> Map<Self, F1, F2>
    where
        F1: Fn(&A) -> &B,
        F2: Fn(&mut A) -> &mut B,
        B: ?Sized,
        Self: Sized;
}

impl<A, T> DerefMutExt<A> for T
where
    T: DerefMut,
{
    fn bimap<B, F1, F2>(self, f1: F1, f2: F2) -> Map<Self, F1, F2>
    where
        F1: Fn(&A) -> &B,
        F2: Fn(&mut A) -> &mut B,
        B: ?Sized,
        Self: Sized,
    {
        Map::new(self, f1, f2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;
    use std::sync::RwLock;

    #[test]
    fn test_box() {
        let boxes = Box::new(1);
        let mapped = boxes.bimap(|x| x, |x| x);
        accept_deref_mut(mapped)
    }

    #[test]
    fn test_lock() {
        let lock = RwLock::new(1);
        let guard = lock.write().unwrap();
        let mapped = guard.bimap(|x| x, |x| x);
        accept_deref_mut(mapped)
    }

    fn accept_deref_mut<D>(_: D)
    where
        D: DerefMut + Deref<Target = i32>,
    {
    }
}
