use std::mem;

fn main() {

    let c = 12345689; //i32 = 32bits = 4 bytes

    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    //c has a reference sign infront if it because we want to know the bytes

    //u8, u16, u32, u64, i8, i16, .... all used to declare integers



}
