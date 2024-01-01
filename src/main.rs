
static GLOBAL_VAR: i32 = 10;
// Integer types
static GLOBAL_I8: i8 = 10;
static GLOBAL_I16: i16 = 100;
static GLOBAL_I32: i32 = 1000;
static GLOBAL_I64: i64 = 10000;
static GLOBAL_I128: i128 = 100000;
static GLOBAL_ISIZE: isize = 1000000;

// Unsigned integer types
static GLOBAL_U8: u8 = 10;
static GLOBAL_U16: u16 = 100;
static GLOBAL_U32: u32 = 1000;
static GLOBAL_U64: u64 = 10000;
static GLOBAL_U128: u128 = 100000;
static GLOBAL_USIZE: usize = 1000000;

// Floating point types
static GLOBAL_F32: f32 = 10.0;
static GLOBAL_F64: f64 = 100.0;

// Boolean type
static GLOBAL_BOOL: bool = true;

// Character type
static GLOBAL_CHAR: char = 'a';

// String type
// Note: String type cannot be static because it does not have a constant expression constructor
// static GLOBAL_STRING: String = String::from("Hello, world!");

// Array type
static GLOBAL_ARRAY: [i32; 5] = [1, 2, 3, 4, 5];

// Tuple type
static GLOBAL_TUPLE: (i32, f64, u8) = (500, 6.4, 1);

fn add_two_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn process_string_and_bool(input: &str, flag: bool) -> bool {
    if flag {
        return input.len() > 5;
    } else {
        return input.contains("rust");
    }
}





fn main() {
    let result = add_two_numbers(GLOBAL_VAR, 7);
    println!("The sum is: {}", result);

    let mut bool = process_string_and_bool("rust", true);
    println!("The result is: {}", bool);

     // Create a String from a string literal
     let mut s = String::from("hello");

     // Append a literal to the String
     s.push_str(", world!");
 
     // s is now "hello, world!"
     println!("{}", s);
 
     // Create a string slice
     let s_slice: &str = &s[0..5];
 
     // s_slice is now "hello"
     println!("{}", s_slice);

      // Define a string slice
    let greeting: &str = "Hello, world!";

    // Use the string slice
    println!("{}", greeting);

    // Define a function that takes a string slice
    fn print_length(s: &str) {
        println!("The length of '{}' is {}.", s, s.len());
    }

    // Call the function with the string slice
    print_length(greeting);
  
}


