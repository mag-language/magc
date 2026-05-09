//! Data structures common to the entire crate, such as [`Token`], [`Expression`] or [`Pattern`].

pub mod expression;
pub mod pattern;
pub mod result;
pub mod token;

use std::collections::HashMap;

pub type Environment = HashMap<String, Box<Expression>>;

pub use self::expression::*;
pub use self::pattern::*;
pub use self::result::*;
pub use self::token::*;
