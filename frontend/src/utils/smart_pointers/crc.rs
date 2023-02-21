use std::alloc::{self, Layout};
use std::any::TypeId;
use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;

// TODO: Sized(?)
pub struct Crc<T>
    where T: Copy + 'static, // We can't call drop() in the raw version
{
    counter: Rc<Cell<u64>>,
    inner: NonNull<T>,
}

impl<T> Crc<T>
    where T: Copy + 'static,
{
    pub fn new(value: T) -> Self {
        Self {
            counter: Rc::new(Cell::new(1)),
            inner: NonNull::new(Box::into_raw(Box::new(value))).unwrap(),
        }
    }

    pub fn as_ref(&self) -> &T {
        unsafe {
            self.inner.as_ref()
        }
    }

    pub fn as_ptr(&self) -> *const () {
        self.inner.as_ptr().cast()
    }

    pub fn clone(value: &Self) -> Self {
        value.counter.update(|val| val + 1);

        Self {
            counter: Rc::clone(&value.counter),
            inner: value.inner,
        }
    }

    pub fn clone_raw(value: &Self) -> RawCrc {
        value.counter.update(|val| val + 1);

        RawCrc {
            counter: Rc::clone(&value.counter),

            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            inner: value.inner.cast(),
        }
    }
}

impl<T> Deref for Crc<T>
    where T: Copy + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.inner.as_ref()
        }
    }
}

impl<T> Drop for Crc<T>
    where T: Copy + 'static,
{
    fn drop(&mut self) {
        if self.counter.update(|val| val - 1) > 0 {
            return;
        }

        unsafe {
            alloc::dealloc(self.inner.as_ptr().cast(), Layout::new::<T>());
        }
    }
}

impl<T> Into<RawCrc> for Crc<T>
    where T: Copy + 'static,
{
    fn into(self) -> RawCrc {
        RawCrc {
            counter: Rc::clone(&self.counter),
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            inner: self.inner.cast(),
        }
    }
}

pub struct RawCrc {
    counter: Rc<Cell<u64>>,

    pub type_id: TypeId,
    pub layout: Layout,
    pub inner: NonNull<()>,
}

impl RawCrc {
    pub fn as_ptr(&self) -> *const () {
        self.inner.as_ptr()
    }

    pub fn downcast<T>(self) -> Result<Crc<T>, ()> 
        where T: Copy,
    {
        if self.type_id != TypeId::of::<T>() {
            return Err(());
        }

        Ok(Crc { 
            counter: Rc::clone(&self.counter),
            inner: self.inner.cast(),
        })
    }

    pub fn clone(value: &Self) -> Self {
        value.counter.update(|val| val + 1);

        RawCrc {
            counter: Rc::clone(&value.counter),

            type_id: value.type_id,
            layout: value.layout,
            inner: value.inner,
        }
    }
}

impl Drop for RawCrc {
    fn drop(&mut self) {
        if self.counter.update(|val| val - 1) > 0 {
            return;
        }
        
        unsafe {
            alloc::dealloc(self.inner.as_ptr().cast(), self.layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc() {
        let a = Crc::new(12345);
        let _b = Crc::clone(&a);
        let _c = Crc::clone_raw(&a);

        assert_eq!(a.counter.get(), 3);
    }
}