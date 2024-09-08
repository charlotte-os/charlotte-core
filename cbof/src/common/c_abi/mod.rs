///! This module contains the C ABI types and functions used to make calls across
///! C ABI boundaries

/// C ABI compatible Result type
#[repr(C)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    /// # Unwrap the result
    /// ## Returns
    /// The inner value if the result is Ok, otherwise panics
    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(_) => panic!("Called unwrap on an Err value"),
        }
    }
}

/// C ABI compatible Option type
#[repr(C)]
pub enum Option<T> {
    Some(T),
    None,
}
