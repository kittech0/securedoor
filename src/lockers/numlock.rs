use std::rc::Rc;

use crate::error::EmptyError;
use crate::lockers::{Locker, NumLock};

impl NumLock {
    pub fn new(key: Rc<u64>) -> Self {
        Self { key }
    }
}

impl Locker for NumLock {
    type Error = EmptyError;
    type Key = Rc<u64>;

    fn unlock(&self, key: Rc<u64>) -> Result<bool, Self::Error> {
        Ok(self.key == key)
    }

    fn lock(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
