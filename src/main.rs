
use std::mem;

fn main() {

    //mutable array that has 5 mutable positions
    let mut a:[i32;5] = [1,2,3,4,5];
    //let mut a = [1,2,3,4,5]; //can also be written like this with specifyin the the data type

    a[0] = 123;
    println!("a has {} elements, first is {}",a.len(), a[0]);

    println!("{:?}",a); //print all contents of an array

    if a == [1,2,3,4,5] {
        println!("match")
    }else{
        println!("does not match")
    }

    let b = [1;10]; //ten elements that are all 1's
    let r = [1u16;10]; //control the size of of bytes for a variable

    for x in b{
        println!("{}",x);
    }

    //another way to write the for loop
    for i in 5..b.len(){
        println!("other for loop {}", i);

    }

    //print out to see how much memory was taken in the array
    println!("b took up {} bytes", mem::size_of_val(&b));
    println!("a took up {} bytes", mem::size_of_val(&a));
    println!("r took up {} bytes", mem::size_of_val(&r));

    //multidimensional array

    let mtx:[[f32;3];3] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.1],
        [0.3, 2.0, 0.2]

    ];
    println!("{:?}", mtx);
    println!("{:?}", mtx[1][1]);

    println!("matrix length is {}", mtx.len());
    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i == j{
                println!("match found mtx [{}][{}] = {}", i, j, mtx[i][j]);
            }
            else {
                println!("no match mtx [{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }

}

