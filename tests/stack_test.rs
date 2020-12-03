extern crate data_structure;
use data_structure::stack::Stack;
#[test]
fn stack_test() {
    let mut stack: Stack<i64> = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(2, *stack.peek().unwrap());
    stack.pop();
    assert_eq!(1, stack.pop().unwrap());
    assert!(stack.is_empty());
    stack.push(22);
    assert!(!stack.is_empty());
    assert_eq!(1, stack.length());
    stack.push(33);
    stack.push(33);
    assert_eq!(3, stack.length());
}

#[test]
#[should_panic]
fn empty_pop() {
    let mut stack: Stack<i64> = Stack::new();
    stack.pop().unwrap();
}
