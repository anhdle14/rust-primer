// Should read: https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
fn main() {
    let mut x = 5; // without mut, variable are immutable by default.
    println!("The value of x: {}", x);
    x = 6;
    println!("The value of x: {}", x);

    // constants can only be declared as an expression, not the result of a function call or any other value that could only be computed at runtime.
    // constants have are literally immutable.
    const MAX_INT8: u8 = 255;
    println!("Defined a const MAX_INT8: {}", MAX_INT8);

    shadowing();
    shadowing_changed_type();
}

// Shadowing is different than marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x: {}", x);
}

fn shadowing_changed_type() {
    let spaces = "   "; // It is a string type.
    let spaces = spaces.len(); // It is an integer now.

    println!("{}", spaces)

    // However, this would not work. Reason is mut will only change the value but not the type.
    // let mut spaces = "   ";
    // spaces = spaces.len();
}

fn data_types() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    // Integer types
    // Length	Signed	Unsigned
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize


    // Character type: Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.

    const C = 'c';
    const Z = 'ℤ';

    // Compound types

    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    println!("The value of tup at index 0: {}", tup.0)

    // Array type. Must be a same type.
    let a = [1, 2, 3, 4, 5];
    let a:[i32; 5] = [1, 2, 3, 4 ,5]; // array only contains 5 items.

}
