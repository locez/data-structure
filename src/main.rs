mod data_structure;
use crate::data_structure::stack::Stack;
fn main() {
    println!("Hello, world!");
    let mut stack: Stack<i64> = Stack::new();
    stack.push(3);
    stack.push(4);
    stack.display();
    stack.pop();
    stack.display();
}
