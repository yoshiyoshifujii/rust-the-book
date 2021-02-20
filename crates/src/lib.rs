//! # My Crates
//!
//! `crates` is a collection of utilities to make performing certain
//! calculations more convenient.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, crates::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}