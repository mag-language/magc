use crate::types::{Pattern};

/// A series of patterns separated by commas.
///
/// The data structure is recursive since the comma is defined as an infix operator. This may
/// look confusing at first, but is fairly easy to work with since you only need to call the
/// method parsing the tuple items recursively.
pub struct TuplePattern {
    pub left:  Box<dyn Pattern>,
    pub right: Box<dyn Pattern>,
}