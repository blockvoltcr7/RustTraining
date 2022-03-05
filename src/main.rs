use std::collections::HashSet;

fn main() {

    //iterations

    let mut vec = vec![3,2,1];

    for x in vec.iter() {
        println!("{} ", x)
    }

    for x in vec.iter_mut() {
        *x +=2;
        println!("{} ", x)
    }

    println!("{:?}", vec);

    //print reverse order
    for x in vec.iter_mut().rev() {
        *x +=2;
        println!(" in reverse {} ", x)
    }




}

