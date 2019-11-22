extern crate nom;
extern crate cse262_project;

use cse262_project::program;

fn main() {
  let result = program("12345");
  println!("{:?}", result);
}
