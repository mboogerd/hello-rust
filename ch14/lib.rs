//! Markdown formatted documentation
//! ================================
//! 
//! Gets compiled into HTML when your crate
//! is published.

/// SillyTrait's ridiculous documentation
/// ============
/// Invisible documentation...
pub use sillymod::SillyTrait;

/// sillymod docs
/// =============
/// Some silly module with silly functions
pub mod sillymod {

    /// This is some function documentation for add_one
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

    /// This is some function documentation for add_two
    pub fn add_two(x: i32) -> i32 {
        x + 2
    }

    /// SillyTrait's ridiculous documentation
    /// ============
    /// Some stuff here...
    pub struct SillyTrait {
        sillyattr: u32
    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, hellocrates::add_one(5));
/// ```
/// 
/// # Panics
/// The scenarios in which the function being documented could panic!. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.
/// 
/// # Errors
/// If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
/// 
/// # Safety
/// If the function is unsafe to call (we discuss unsafety in Chapter 19), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.
/// 
pub fn add_one(x: i32) -> i32 {
    x + 1
}