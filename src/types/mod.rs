pub mod expression;
pub mod token;
pub mod pattern;
pub mod result;

use std::collections::HashMap;

pub type Environment = HashMap<String, Box<Expression>>;

pub use self::expression::*;
pub use self::token::*;
pub use self::pattern::*;
pub use self::result::*;