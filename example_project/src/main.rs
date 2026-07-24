use crate::bounded_stack::BoundedStack;

pub mod bounded_stack;

fn main() {
    let mut stack: BoundedStack<_, 30> = BoundedStack::new();
    stack.push(3 as u64);
    println!("Hello, world! ");
}
