
// pub type Error = Box<dyn std::error::Error>;

// TODO: user "thiserror"?
#[derive(Debug)]
pub enum Error {
    S0102, // Number out of range: {{token}}
}

pub type Result<T> = core::result::Result<T, Error>;
