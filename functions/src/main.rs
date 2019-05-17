fn main() {
    function();

    function_with_arg(42);

    function_with_return_value(42);
}

fn function() {
    println!("This is a function with no arguments and no return value");
}

fn function_with_arg(input: i32) {
    println!("This is a function with 1 argument: {}", input);
}

fn function_with_return_value(input: i32) -> i32 {
    input * input
}
