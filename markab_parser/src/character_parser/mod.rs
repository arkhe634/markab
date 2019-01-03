mod error;
mod parser;
mod utility;

pub use self::{
	error::CharacterParserError,
	parser::CharacterParser,
	utility::character,
};
