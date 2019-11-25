use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}


struct Runtime {
  functions: HashMap<String, Vec<Node>>,
  stack: Vec<HashMap<String, Value>>,
}

impl Runtime {

  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  pub fn run(&mut self, node: &Node) -> Result<Value, &'static str> {
    match node {
      Node::Program{children} => {
        for n in children {
          match n {
            Node::FunctionDefine{..} => {
              self.run(n);
            },
            _ => (),
          }
        }
        Err("Program")
      },
      Node::FunctionCall{name, children} => {
        self.stack.push(HashMap::new());
        let mut result: Result<Value, &'static str> = Err("Undefined function");
        match self.functions.get(name) {
          Some(statements) => {
            println!("{:?}", statements);
            for n in statements.clone() {
              result = self.run(&n);
            }
          },
          None => (),
        };
        self.stack.pop();
        result
      },
      Node::FunctionDefine{children} => {
        let (head, tail) = children.split_at(1);
        match &head[0] {
          Node::Identifier{value} => {
            self.functions.insert(value.to_string(), tail.to_vec());
          },
          _ => (),
        }
        Err("Function Define")
      },
      Node::FunctionReturn{children} => {
        self.run(&children[0])
      },
      Node::Identifier{value} => {
        let last = self.stack.len() - 1;
        match self.stack[last].get(value) {
          Some(id_value) => Ok(id_value.clone()),
          None => Err("Undefined variable"),
        }
      },
      Node::Statement{children} => {
        match children[0] {
          Node::VariableDefine{..} |
          Node::FunctionReturn{..} => {
            self.run(&children[0])
          },
          _ => Err("Unknown Statement"),
        }
      },
      Node::VariableDefine{children} => {
        // Variable name
        let name: String = match &children[0] {
          Node::Identifier{value} => value.clone(),
          _ => "".to_string(),
        };
        // Expression result
        let value = self.run(&children[1]).unwrap();
        let last = self.stack.len() - 1;
        self.stack[last].insert(name, value.clone());
        Ok(value)
      }
      Node::Expression{children} => {
        match children[0] {
          Node::MathExpression{..} |
          Node::Number{..} |
          Node::Identifier{..} => {
            self.run(&children[0])
          },
          _ => Err("Unknown Expression"),
        }
      }
      Node::Number{value} => {
        Ok(Value::Number(*value))
      }
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      }
      _ => {
        println!("{:?}", node);
        Err("Unhandled Nodee")
      },
    }
  }

}

pub fn run(node: &Node) -> Result<Value, &'static str> {
  let mut runtime = Runtime::new();
  runtime.run(node)
  println!("{:?}\n{:?}", runtime.stack, runtime.functions);
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  runtime.run(&start_main)
}