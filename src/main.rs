#![feature(type_alias_impl_trait)]
#![feature(inherent_associated_types)]

use std::rc::Rc;

use crate::door::Door;
use crate::error::BoxError;
use crate::lockers::NumLock;

mod door;
mod error;
mod lockers;

fn main() -> Result<(), BoxError> {
    let key = Rc::new(12345);
    let door = Door::new(NumLock::new(key.clone()));
    let opened = door.unlock(key.clone())?;
    if let Ok(door) = opened {
        println!("Door is opened");
        door.lock()?;
    } else {
        println!("Door is closed");
    }
    Ok(())
}
