use std::marker::PhantomData;

use crate::lockers::Locker;

pub struct Open;
pub struct Close;

pub struct Door<L: Locker, S> {
    locker: L,
    state: PhantomData<S>,
}

impl<L: Locker, S> Door<L, S> {
    pub type Opened = Door<L, Open>;
    pub type Closed = Door<L, Close>;
}

impl<L: Locker> Door<L, Close> {
    pub fn new(locker: L) -> Self {
        Self {
            locker,
            state: PhantomData,
        }
    }
    pub fn unlock(self, key: L::Key) -> Result<Result<Self::Opened, Self::Closed>, L::Error> {
        if self.locker.unlock(key)? {
            Ok(Ok(Door {
                locker: self.locker,
                state: PhantomData,
            }))
        } else {
            Ok(Err(Door {
                locker: self.locker,
                state: PhantomData,
            }))
        }
    }
}

impl<L: Locker> Door<L, Open> {
    pub fn lock(self) -> Result<Self::Closed, L::Error> {
        self.locker.lock()?;
        Ok(Door {
            locker: self.locker,
            state: PhantomData,
        })
    }
    fn get_mutable_locker(&mut self) -> &mut L {
        &mut self.locker
    }
}
