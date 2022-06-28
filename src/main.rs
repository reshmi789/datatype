/*
Rust is a statically typed language, 
which means that it must know the types of all variables at compile time. 
The compiler can usually infer what type we want to use based on the value and how we use it. 
In cases when many types are possible, such as when 
we converted a String to a numeric type using parse
*/

/*
Scalar Types
A scalar type represents a single value. 
Rust has four primary scalar types: 
 - integers, 
 - floating-point numbers, 
 - Booleans, 
 - characters.
*/

/*
Integer Types
An integer is a number without a fractional component.
*/
/*
Integer Types in Rust
==========================
Length	Signed	Unsigned
8-bit	i8  	u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize
===========================
the number needs to have a sign with it (signed) 
Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, 
where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. 

whether it will only ever be positive and can be represented without a sign (unsigned).
Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.
*/
// =========================
/*
Integer Overflow
Let’s say you have a variable of type u8 that can hold values between 0 and 255. 
If you try to change the variable to a value outside of that range, such as 256, integer overflow will occur.
Rust does not include checks for integer overflow that cause panics. 
Instead, if overflow occurs, Rust performs two’s complement wrapping. 
In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. 
In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. 
The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. 
*/

// ================================
// ================================

/*
Floating-Point Types
numbers with decimal points
- Rust’s floating-point types are 
    - f32 - f32 type is a single-precision float
    - f64, which are 32 bits and 64 bits in size, respectively -f64 has double precision. 
- The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. 
- All floating-point types are signed.
*/

/*
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
*/

// ================================
// ================================

/*
The Boolean Type
two possible values: 
    - true 
    - false
Booleans are one byte in size.
Boolean type in Rust is specified using bool.
*/

/*
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
*/

// ================================
// ================================

/*
The Character Type
single quotes
four bytes in size 
represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII
*/

/*
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
*/

// ================================
// ================================

/*
Compound Types
Compound types can group multiple values into one type. 
two primitive compound types: 
- tuples 
- arrays.

    The Tuple Type
    - grouping together a number of values with a variety of types into one compound type
    - have a fixed length
    - once declared, they cannot grow or shrink in size
    - Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same

    fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    }

    destructure a tuple value

    fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    }

    creates the tuple x and then makes new variables for each element by using their respective indices. 
    As with most programming languages, the first index in a tuple is 0.

    fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    }

    The tuple without any values, (), is a special type that has only one value, also written (). 
    The type is called the unit type and the value is called the unit value. 
    Expressions implicitly return the unit value if they don’t return any other value.
    */

    // ================================



// ================================
// ================================

/*

*/

fn main() {
    println!("Hello, world!");
}
