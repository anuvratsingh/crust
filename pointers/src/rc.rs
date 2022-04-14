use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
// Reference Counted Pointer, a bookkeeper knowing who has borrowed what and when.
// +Shared access to values
// -Size Increase
// -Runtime Cost
// -Not Threadable
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}
impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        // Rc { inner: &*inner } // Compiles but is wrong, as function return Box(new inner) is freed
        // Rc becomes invalid therefore deref becomes invalid
        Rc {
            // SAFETY: Box does not give us a null pointer.
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) }, // Now it doesn't deref it
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFTEY: self.inner is a Box that is only dellocated when the last Rc goes away
        // we have an Rc, therefore the Box has not been dealloacted, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            drop(inner);
            // SAFETY: we are the _only_ Rc left, and we are being dropped.
            // Therefore, after us, there will be no Rc's, and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // There are other Rcs, so count -1 and don't drop
            inner.refcount.set(c - 1);
        }
    }
}
