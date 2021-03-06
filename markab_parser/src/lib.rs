//! This crate provides simple, copy-less and rich-error-message parser combinator for parsing string.
//!
//! # Create simple parser
//! You can create simple parser with [character], [character_class] and [string].
//!
//! Each methods returns a class implementing [Parser] trait,
//! [CharacterParser], [CharacterClassParser], [StringParser],
//! and returns a slice of the source string by calling the [parse] method of Parser trait.
//!
//! [character]: fn.character.html
//! [character_class]: fn.character_class.html
//! [string]: fn.string.html
//! [Parser]: trait.Parser.html
//! [CharacterParser]: character_parser/struct.CharacterParser.html
//! [CharacterClassParser]: character_class_parser/struct.CharacterClassParser.html
//! [StringParser]: string_parser/struct.StringParser.html
//! [parse]: trait.Parser.html#tymethod.parse
//! ```
//! use markab_parser::{
//! 	string,
//! 	Parser,
//! 	};
//!
//! let src = "requirement";
//! let mut pos = 0;
//! let parser = string("requirement");
//! let result = parser.parse(src, &mut pos);
//! assert!(result.is_ok());
//! assert_eq!(result.ok().unwrap(), "requirement");
//! assert_eq!(pos, 11);
//! ```
//!
//! # Combinate parser
//! [Parser] trait has methods for parser combination.
//!
//! ```
//! use markab_parser::{
//! 	character,
//! 	character_class,
//! 	Parser,
//! 	};
//!
//! // require "1" and return "1"
//! let parser = character('1');
//! // require "12" and return ("1","2")
//! let seq = parser.and_then(character('2'));
//! // require "1" or "2" and return Either("1","2")
//! let ord = character_class(false, &[], &['1'..'2']);
//! // require [0-9]+ and return as usize
//! let map = character_class(false, &[], &['0'..'9'])
//! 	.one_or_more()
//! 	.stringify()
//! 	.map(&|num| num.parse::<usize>().unwrap());
//! ```
//!
//! # Create new parser
//! If the parser has parameters, you should implement [Parser] trait.
//! If the parser does not have parameters, you should implement [Parseable] trait.
//! Parseable trait provides static method [Parseable::parse] and
//! [Parseable::get_parser] to get a parser instance for parser combination.
//! Here is an example of [Parseable] class `WS`, which parses whitespace characters.
//!
//! [Parseable]: trait.Parseable.html
//! [Parseable::parse]: trait.Parseable.html#tymethod.parse
//! [Parseable::get_parser]: trait.Parseable.html#method.get_parser
//!
//! ```
//! use markab_parser::{
//! 	character_class,
//! 	character_class_parser::CharacterClassParser,
//! 	repetition_parser::RepetitionParserError,
//! 	Parseable,
//! 	Parser,
//! 	};
//!
//! pub struct WS;
//!
//! impl<'a> Parseable<'a> for WS
//! 	{
//! 	type Error = RepetitionParserError<'a, CharacterClassParser<'a>>;
//! 	type Output = ();
//!
//! 	fn parse(src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
//! 		{
//! 		character_class(false, &[' ', '\t', '\n', '\r'], &[])
//! 			.one_or_more()
//! 			.skip(src, pos)
//! 		}
//!
//! 	fn name() -> &'a str
//! 		{
//! 		"WS"
//! 		}
//! 	}
//! ```

pub mod and_parser;
pub mod character_class_parser;
pub mod character_parser;
mod equal;
mod error;
pub mod gen_parser;
pub mod map_parser;
pub mod not_parser;
pub mod order_parser;
mod parseable;
pub mod parseable_parser;
mod parser;
pub mod repetition_parser;
pub mod sequence_parser;
pub mod string_parser;
pub mod stringify_parser;

pub use crate::{
	and_parser::and,
	character_class_parser::character_class,
	character_parser::character,
	error::Error,
	not_parser::not,
	parseable::Parseable,
	parser::Parser,
	string_parser::string,
};
