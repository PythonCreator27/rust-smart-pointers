use crate::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });
        Rc {
            // SAFETY: Box does not give a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        // we have an Rc, therfore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            drop(inner);
            // SAFETY: we are the _only_ RC left and we are being dropped.
            // therefore, after this, there will be no Rc's and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.refcount.set(c - 1)
        }
    }
}
