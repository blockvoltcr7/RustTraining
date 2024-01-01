
static GLOBAL_VAR: i32 = 10;


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
}


