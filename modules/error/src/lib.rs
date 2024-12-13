
// TODO: user "thiserror"?
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    T2001,
    T2002,
    S0102,
    D1002, // Number out of range: {{token}}
}

pub type Result<T> = core::result::Result<T, Error>;
