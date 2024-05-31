use std::error::Error;
use std::rc::Rc;

pub mod numlock;

pub trait Locker {
    type Error: Error;
    type Key: Clone;
    fn unlock(&self, key: Self::Key) -> Result<bool, Self::Error>;
    fn lock(&self) -> Result<(), Self::Error>;
}

pub struct NumLock {
    key: Rc<u64>,
}
