//! This module contains the C ABI types and functions used to make calls across
//! C ABI boundaries

use core::panic;

/// C ABI compatible Result type
#[repr(C)]
pub enum Result<T, E> {
    /// Indicates the operation was successful
    Ok(T),
    /// Indicates the operation failed
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
    /// # Expect the result
    /// ## Returns
    /// The inner value if the result is Ok
    /// ## Panics
    /// Panics with the provided message if the result is Err
    pub fn expect(self, msg: &'static str) -> T {
        if let Self::Ok(val) = self {
            val
        } else {
            panic!("{}", msg)
        }
    }
}

/// C ABI compatible Option type
#[repr(C)]
pub enum Option<T> {
    /// Indicates the value is present
    Some(T),
    /// Indicates the value is absent
    None,
}
