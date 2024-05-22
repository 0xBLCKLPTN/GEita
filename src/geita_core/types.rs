use std::error::Error;

pub type BoxedResult<T> = Result<T, Box<dyn Error>>;
