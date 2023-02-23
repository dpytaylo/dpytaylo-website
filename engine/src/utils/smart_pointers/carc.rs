// use std::alloc::{self, Layout};
// use std::any::TypeId;
// use std::ops::Deref;
// use std::ptr::NonNull;
// use std::sync::Arc;
// use std::sync::atomic::{AtomicU64, Ordering};

// // TODO: Sized(?)
// pub struct Carc<T>
//     where T: Copy, // We can't to call drop() in the raw version
// {
//     counter: Arc<AtomicU64>,
//     inner: NonNull<T>,
// }

// impl<T> Carc<T> {
//     pub fn new(value: T) -> Self {
//         Self {
//             counter: Arc::new(AtomicU64::new(1)),
//             inner: NonNull::new(Box::into_raw(Box::new(value))),
//         }
//     }

//     pub fn as_ref(&self) -> &T {
//         unsafe {
//             self.inner.as_ref()
//         }
//     }

//     pub fn as_ptr(&self) -> *mut T {
//         self.inner.as_ptr()
//     }

//     pub fn clone(value: &Self) -> Self {
//         // From Arc clone() function
//         let old_size = value.counter.fetch_add(1, Ordering::Relaxed);
//         assert_ne!(old_size, u64::MAX, "pointer counter overflow");

//         Carc {
//             counter: Arc::clone(&value.counter),
//             inner: value.inner,
//         }
//     }

//     pub fn clone_raw(value: &Self) -> RawCarc {
//         // From Arc clone() function
//         let old_size = value.counter.fetch_add(1, Ordering::Relaxed);
//         assert_ne!(old_size, u64::MAX, "pointer counter overflow");

//         RawCarc {
//             counter: Arc::clone(&value.counter),

//             type_id: TypeId::of::<T>(),
//             layout: Layout::new::<T>(),
//             inner: value.inner,
//         }
//     }
// }

// impl<T> Deref for Carc<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         unsafe {
//             self.inner.as_ref()
//         }
//     }
// }

// impl<T> Drop for Carc<T> {
//     fn drop(&mut self) {
//         // TODO check Ordering(?)
//         if self.counter.fetch_sub(1, Ordering::Release) > 0 {
//             return;
//         }

//         unsafe {
//             alloc::dealloc(self.inner.as_ptr(), Layout::new::<T>());
//         }
//     }
// }

// pub struct RawCarc {
//     counter: Arc<AtomicU64>,

//     pub type_id: TypeId,
//     pub layout: Layout,
//     pub inner: NonNull<()>,
// }

// impl RawCarc {
//     pub fn as_ptr(&self) -> *mut () {
//         self.inner.as_ptr()
//     }

//     pub fn clone(value: &Self) -> Self {
//         // From Arc clone() function
//         let old_size = value.counter.fetch_add(1, Ordering::Relaxed);
//         assert_ne!(old_size, u64::MAX, "pointer counter overflow");

//         RawCarc {
//             counter: Arc::clone(&value.counter),

//             type_id: value.type_id,
//             layout: value.layout,
//             inner: value.inner,
//         }
//     }
// }

// impl Drop for RawCarc {
//     fn drop(&mut self) {
//         // TODO check Ordering(?)
//         if self.counter.fetch_sub(1, Ordering::Release) > 0 {
//             return;
//         }
        
//         unsafe {
//             alloc::dealloc(self.inner.as_ptr(), self.layout);
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn carc() {
//         let a = Carc::new(12345);
//         let b = Carc::clone(&a);
//         let c = Carc::clone_raw(&a);

//         assert_eq!(a.counter.load(Ordering::Acquire), 3);
//     }
// }