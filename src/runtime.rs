use crate::parser::Node;

#[derive(Debug, PartialEq)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

pub fn run(program: Node) -> Result<Value, &'static str> {
  Err("Not implemented")
}