# markab_parser

This crate provides simple, copy-less and rich-error-message parser combinator for parsing string.

# Create simple parser
You can create simple parser with `character`, `character_class` and `string`.

Each methods returns a class implementing `Parser` trait,
`CharacterParser`, `CharacterClassParser`, `StringParser`,
and returns a slice of the source string by calling the `parse` method of Parser trait.

```
use markab_parser::{
	string,
	Parser,
	};

let src = "requirement";
let mut pos = 0;
let parser = string("requirement");
let result = parser.parse(src, &mut pos);
assert!(result.is_ok());
assert_eq!(result.ok().unwrap(), "requirement");
assert_eq!(pos, 11);
```

# Combinate parser
`Parser` trait has methods for parser combination.

```
use markab_parser::{
	character,
	character_class,
	Parser,
	};

// require "1" and return "1"
let parser = character('1');
// require "12" and return ("1","2")
let seq = parser.and_then(character('2'));
// require "1" or "2" and return Either("1","2")
let ord = character_class(false, &``, &`'1'..'2'`);
// require `0-9`+ and return as usize
let map = character_class(false, &``, &`'0'..'9'`)
	.one_or_more()
	.stringify()
	.map(|num| num.parse::<usize>().unwrap());
```

# Create new parser
If the parser has parameters, you should implement `Parser` trait.
If the parser does not have parameters, you should implement `Parseable` trait.
Parseable trait provides static method `Parseable::parse` and
`Parseable::get_parser` to get a parser instance for parser combination
