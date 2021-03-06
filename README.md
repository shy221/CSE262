# Parser Project

In this project you will write a grammar and a parser that recognizes the grammar. I've provided a number of statements in the language in `src/parser.rs`, which runs tests against your code. There are 28 tests you will need to pass to get full credit.

An example program in the language looks like this:

```
fn foo(a,b,c) {
  let x = a + 1; 
  // This is a comment
  let y = bar(c - b);
  return x * y; // Multiply the results
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}
```

This is the full language. You can write functions, call functions, do math, and return values. Strings and Booleans are also data types in the language, but the runtime doesn't do anything with them.

The output of your parser is a tree of Nodes. You can find a list of Node types in `src/parser.rs` The tree should contain a single root "Program" node. The rest of the tree you will have to determine using the test cases, as well as the runtime found in `src/runtime.rs` (you won't have to modify this file). You can use this runtime to reverse engineer the parser in a sense. *i.e.* the runtime works on the result of the parser, so whatever your parser produces it needs to be in a form the runtime works with.

## Parser Combinators

We will be using the [nom](https://crates.io/crates/nom) parser combinator library in Rust. A bit about combinators:

Parser combinators are an approach to parsers that is very different from software like lex and yacc. Instead of writing the grammar in a separate syntax and generating the corresponding code, you use very small functions with a very specific purpose, like "take 5 bytes", or "recognize the word 'HTTP'", and assemble then in meaningful patterns like "recognize 'HTTP', then a space, then a version". The resulting code is small, and looks like the grammar you would have written with other parser approaches.

This gives us a few advantages:

- the parsers are small and easy to write
- the parsers components are easy to reuse (if they're general enough, please add them to nom!)
- the parsers components are easy to test separately (unit tests and property-based tests)
- the parser combination code looks close to the grammar you would have written
- you can build partial parsers, specific to the data you need at the moment, and ignore the rest
- Here is an example of one such parser, to recognize text between parentheses:

```rust
use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::char,
  bytes::complete::is_not
};

fn parens(input: &str) -> IResult<&str, &str> {
  delimited(char('('), is_not(")"), char(')'))(input)
}
```

It defines a function named parens which will recognize a sequence of the character (, the longest byte array not containing ), then the character ), and will return the byte array in the middle.

Here is another parser, written without using nom's combinators this time:

```rust
#[macro_use]
extern crate nom;

use nom::{IResult, Err, Needed};

fn take4(i: &[u8]) -> IResult<&[u8], &[u8]>{
  if i.len() < 4 {
    Err(Err::Incomplete(Needed::Size(4)))
  } else {
    Ok((&i[4..], &i[0..4]))
  }
}
```

This function takes a byte array as input, and tries to consume 4 bytes. Writing all the parsers manually, like this, is dangerous, despite Rust's safety features. There are still a lot of mistakes one can make. That's why nom provides a list of function and macros to help in developing parsers.

With functions, you would write it like this:

```rust
use nom::{IResult, bytes::streaming::take};
fn take4(input: &str) -> IResult<&str, &str> {
  take(4u8)(input)
}
```

A parser in nom is a function which, for an input type I, an output type O and an optional error type E, will have the following signature:

```rust
fn parser(input: I) -> IResult<I, O, E>;
```

Or like this, if you don't want to specify a custom error type (it will be u32 by default):

```rust
fn parser(input: I) -> IResult<I, O>;
```

IResult is an alias for the Result type:

```rust
use nom::{Needed, error::ErrorKind};

type IResult<I, O, E = (I,ErrorKind)> = Result<(I, O), Err<E>>;

enum Err<E> {
  Incomplete(Needed),
  Error(E),
  Failure(E),
}
```

It can have the following values:

- a correct result Ok((I,O)) with the first element being the remaining of the input (not parsed yet), and the second the output value;
- an error Err(Err::Error(c)) with c an error that can be built from the input position and a parser specific error
- an error Err(Err::Incomplete(Needed)) indicating that more input is necessary. Needed can indicate how much data is needed
- an error Err(Err::Failure(c)). It works like the Error case, except it indicates an unrecoverable error: we cannot backtrack and test another parser

Please refer to the ["choose a combinator"](https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md) guide for an exhaustive list of parsers. See also the rest of the documentation [here](https://docs.rs/nom/5.0.1/nom/).

## Deliverables

### Parser

Finish the parser that's started in `/src/parser.rs`. The input is passed into the function `program()` as a `&str`, and that kicks off the parsing process. The way program is currently defined, it states that a program is either a number or an identifier. These are the names of other functions, each which conforms to the same function signature: 

```rust
pub fn combinator_name(input: &str) -> IResult<&str, Node>
```

The file defines two other combinators `number()` and `identifier()`. You'll need to write other combinators for the various other nodes in the grammar, and tie them together using the various parser combinators that are included in the nom library. You can find a list of the built-in combinators and what they do [here](https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md).

Here is an example combinator in your grammar.

```rust
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, expression) = expression(input)?;
  Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
}
```

The parser is automatically tested in `tests/parser.rs`. There are 28 different statements in the language and their expected output from the runtime. In order to get full credit, you'll need to pass all 28 tests.

### Grammar

Along with the parser you should write an ebnf grammar for the language in `/grammar.ebnf`. This file already contains the nodes in the grammar you need to implement. Use the EBNF format. Note, if you do this correctly, you'll find the EBNF grammar maps almost 1:1 with the parer combinators you'll write. Furthermore, the semantics of EBNF (such as optional nodes, repeated nodes, choice nodes, etc.) map directly to combinators provided by `nom`. You'll find even more options in the linked reference.
