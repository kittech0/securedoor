pub type BoxError = Box<dyn std::error::Error>;
#[derive(thiserror::Error, Debug)]
pub enum EmptyError {}
