fn main() {
    println!("EVM implmentation");

    let mut stack = Vec::new();

    add_to_stack(&mut stack, 4);
    add_to_stack(&mut stack, 6);
    add_to_stack(&mut stack, 6);
    remove_from_stack(&mut stack);

    println!("Stack {:?}", stack);
}

fn add_to_stack(stack: &mut Vec<i32>, value: i32) {
    stack.push(value);
}

fn remove_from_stack(stack: &mut Vec<i32>) {
    stack.pop();
}
