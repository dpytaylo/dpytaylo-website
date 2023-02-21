use std::alloc::{Layout, self};
use std::any::TypeId;
use std::fmt::Pointer;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

impl ResourceVec {
    pub fn new<T>(value: Vec<T>) -> Self 
        where T: 'static,
    {
        Self {
            type_id: TypeId::of::<T>(),
            type_size: usize,
            inner: NonNull::new(
                Box::into_raw(Box::new(value)) as *mut (),
            ).unwrap(),
            size: usize,
            cap: usize,

            counter: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn new_pointer<T>(&self) -> Result<Res<T>, ()>
        where T: 'static,
    {
        if self.type_id != TypeId::of::<T>() {
            do yeet ();
        }

        // From Arc clone() function
        let old_size = self.counter.fetch_add(1, Ordering::Relaxed);
        assert_ne!(old_size, u64::MAX, "pointer counter overflow");

        Ok(Res::new(
            NonNull::new(self.inner.as_ptr() as *mut T).unwrap(),
            Arc::clone(&self.counter),
        ))
    }
}

impl Drop for ResourceVec {
    fn drop(&mut self) {
        // TODO check Ordering(?)
        if self.counter.load(Ordering::SeqCst) != 1 {
            return;
        }

        // assert_eq!(val, 1, "invalid count of pointer owners");
        
        unsafe {
            // Check (?)
            alloc::dealloc(self.inner.as_ptr(), self.layout);
        }
    }
}

pub struct VecRes<T> {
    inner: NonNull<T>,
    counter: Arc<AtomicU64>,
}

impl<T> VecRes<T> {
    fn new(inner: NonNull<T>, counter: Arc<AtomicU64>) -> Self {
        Self {
            inner,
            counter,
        }
    }

    pub fn clone(resource: &Self) -> Self {
        let old_size = resource.counter.fetch_add(1, Ordering::Relaxed);
        assert_ne!(old_size, u64::MAX, "pointer counter overflow");

        Self {
            inner: resource.inner,
            counter: Arc::clone(&resource.counter),
        }
    }
}

impl<T> Deref for VecRes<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.inner.as_ref()
        }
    }
}

impl<T> Drop for VecRes<T> {
    fn drop(&mut self) {
        // TODO check Ordering(?)
        if self.counter.fetch_sub(1, Ordering::SeqCst) != 1 {
            return;
        }

        unsafe {
            let _ = Box::from_raw(self.inner.as_ptr());
        }
    }
}

impl<T> Pointer for VecRes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self.inner.as_ptr();
        Pointer::fmt(&ptr, f)
    }
}