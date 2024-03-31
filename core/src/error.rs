pub enum Error {
    Undefined,
}

pub type Result<T> = core::result::Result<T, Error>;
