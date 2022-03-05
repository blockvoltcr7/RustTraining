use std::collections::HashSet;


fn multiply(x:i32, y:i32) -> i32{ //arrow symbol is used to specify return statement, must specify data type being returned

    x * y //return statement does not require a semi colon

}

fn main() {

    let a = 5;
    let b = 5;

    let num = multiply(a,b);

    println!("{}", num)

}

