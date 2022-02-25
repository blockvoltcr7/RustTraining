fn main() {


    let a: u8 = 255; // - u8 unsined integer, which means that the variable a can only be between 0 and 255, 0 to 2^N-1


    println!("a = {}", a);

    //all the variables you declare are immutable

    // you have to be explicit about creating a variable that can be assigned to different values
    //you have to use a keyword called 'mut' in front of the variable


    //i = signed: range of -128 to 127, so b can only hold within that range.
    let mut b: i8 = 0;

    b = 20;

    println!("b = {}",b);

}
