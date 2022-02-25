use std::mem;

fn main() {

    let d: char = 'x'; //32 bit unicode char
    //this char takes up 32 bits as a unicode character

    println!("{} is a char, size = {} bytes",d, mem::size_of_val(&d));


}
