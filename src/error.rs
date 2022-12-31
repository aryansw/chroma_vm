use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Register: {0}")]
    InvalidRegister(u8),
}
