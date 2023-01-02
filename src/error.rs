use image::Rgb;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Register: {0}")]
    InvalidRegister(u8),
    #[error("Pixel at (x, y) not Present in Image")]
    PixelNotPresent(u32, u32),
}
