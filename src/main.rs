use std::mem;

fn main() {

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
             z, size_of_z, size_of_z*8
    );


}
