use std::collections::HashSet;


fn increase (x: &mut i32){
    *x += 1;
}

fn main() {

    let mut num = 1;
    increase(&mut num);
    println!("{}", num);

}

