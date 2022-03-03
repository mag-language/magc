#[derive(Debug)]
pub enum Pattern {
	/// A pattern ẁhich contains a variable name, and an optional type. Examples: `count`,
	/// ´_ Int´, `name String`
	VariablePattern {
		name: String,
		type_id: Option<String>,
	},

	/// A structure which encloses comma-separated patterns in braces.
	TuplePattern {
		children: Vec<Pattern>,
	},

	/// A structure which encloses comma-separated patterns in brackets.
	ArrayPattern {
		children: Vec<Pattern>,
	},

	/// A structure which represents the fields of a Mag object
	RecordPattern {
		records: Vec<Record>,
	}
}

#[derive(Debug)]
pub struct Record {
	name: String,
	value: Option<Box<Expression>>,
}


pub enum Literal {
    Boolean(boolean),
    Integer(i32),
    Float(f64),
    String(String),
}