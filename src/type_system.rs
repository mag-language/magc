/// An interface which is implemented by anything that has a type.
pub trait Typed {
    fn can_assign_from(&self, other: Box<dyn Typed>) -> bool;
    fn get_member_type(&self, name: String) -> Option<Box<dyn Typed>>;
}