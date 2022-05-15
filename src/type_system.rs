//! A set of utilities which enable static type checking in Mag.

/// An interface which is implemented by anything that has a type.
pub trait Typed {
    // / Can we assign a value of the given type to a variable of this type?
    // fn can_assign_from(&self, other: Box<dyn Typed>) -> bool;
    /// Get the type of this object as a string.
    fn get_type(&self) -> Option<String>;
}