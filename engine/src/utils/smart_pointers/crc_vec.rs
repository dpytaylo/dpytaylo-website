use std::alloc::{self, Layout};
use std::any::TypeId;
use std::cell::Cell;
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;
use std::rc::Rc;

// TODO: Sized(?)
pub struct CrcVec<T>
    where T: Copy + 'static, // We can't call drop() in the raw version
{
    counter: Rc<Cell<u64>>,
    inner: NonNull<T>,
    pub size: usize,
    pub capacity: usize,
}

impl<T> CrcVec<T>
    where T: Copy + 'static,
{
    pub fn new(value: Vec<T>) -> Self {
        let (ptr, size, capacity) = value.into_raw_parts();

        Self {
            counter: Rc::new(Cell::new(1)),
            inner: NonNull::new(ptr).unwrap(),
            size,
            capacity,
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clone(value: &Self) -> Self {
        value.counter.update(|val| val + 1);

        Self {
            counter: Rc::clone(&value.counter),
            inner: value.inner,
            size: value.size,
            capacity: value.capacity,
        }
    }

    pub fn clone_raw(value: &Self) -> RawCrcVec {
        value.counter.update(|val| val + 1);

        RawCrcVec {
            counter: Rc::clone(&value.counter),

            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),

            inner: NonNull::new(value.inner.as_ptr().cast()).unwrap(),

            size: value.size,
            capacity: value.capacity,
        }
    }
}

impl<T> Index<usize> for CrcVec<T>
    where T: Copy + 'static,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "index out of bounds: the len is {} but the index is {}", self.size, index);

        unsafe { 
            &*self.as_ptr().add(index)
        }
    }
}

impl<T> IndexMut<usize> for CrcVec<T>
    where T: Copy + 'static,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size, "index out of bounds: the len is {} but the index is {}", self.size, index);

        unsafe { 
            &mut *self.as_ptr().add(index).cast_mut()
        }
    }
}

impl<T> Drop for CrcVec<T>
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

impl<T> Into<RawCrcVec> for CrcVec<T>
    where T: Copy + 'static,
{
    fn into(self) -> RawCrcVec {
        RawCrcVec {
            counter: Rc::clone(&self.counter),
            
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            inner: self.inner.cast(),

            size: self.size,
            capacity: self.capacity,
        }
    }
}

impl<T> From<Vec<T>> for CrcVec<T>
    where T: Copy + 'static,
{
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

pub struct RawCrcVec {
    counter: Rc<Cell<u64>>,

    pub type_id: TypeId,
    pub layout: Layout,

    pub inner: NonNull<()>,

    pub size: usize,
    pub capacity: usize,
}

impl RawCrcVec {
    pub fn as_ptr(&self) -> *const () {
        self.inner.as_ptr()
    }

    pub fn downcast<T>(self) -> Result<CrcVec<T>, ()> 
        where T: Copy,
    {
        if self.type_id != TypeId::of::<T>() {
            return Err(());
        }

        Ok(CrcVec { 
            counter: Rc::clone(&self.counter),
            inner: self.inner.cast(),
            size: self.size,
            capacity: self.capacity, 
        })
    }

    pub fn clone(value: &Self) -> Self {
        value.counter.update(|val| val + 1);

        RawCrcVec {
            counter: Rc::clone(&value.counter),

            type_id: value.type_id,
            layout: value.layout,
            inner: value.inner,

            size: value.size,
            capacity: value.capacity,
        }
    }
}

impl Drop for RawCrcVec {
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
    fn crc_vec() {
        let a = CrcVec::new(vec![12345]);
        let _b = CrcVec::clone(&a);
        let _c = CrcVec::clone_raw(&a);

        assert_eq!(a.counter.get(), 3);
    }
}