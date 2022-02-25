use std::mem;


const MY_CONSTANT_VAR:u8 = 12; //global variables
static mut z:i32 = 123;

fn main() {

    println!("{}", MY_CONSTANT_VAR);

    unsafe{
        z = 77;
        println!("{}", z);
    }

}
