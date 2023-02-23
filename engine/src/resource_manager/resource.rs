use crate::utils::smart_pointers::crc_vec::{CrcVec, RawCrcVec};

pub struct Resource<T> {
    inner: T,
}

impl Resource<RawCrcVec> {
    pub fn new<T>(value: Vec<T>) -> (Self, CrcVec<T>)
        where T: Copy,
    {
        let ptr = CrcVec::new(value);
        let inner = CrcVec::clone_raw(&ptr);

        (Self { inner }, ptr)
    }

    pub fn clone_ptr<T>(&self) -> Result<CrcVec<T>, ()>
        where T: Copy,
    {
        RawCrcVec::clone(&self.inner).downcast::<T>()
    }
}