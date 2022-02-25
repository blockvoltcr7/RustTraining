
fn main() {


    let mut x = 1;

    while x < 100{
        x *= 2;
        println!("x = {}", x);
    }

    //other version of while loop called loop which is a 'while true'

    let mut y = 1;

    loop{
        y*=2;
        println!("y = {}", y);
        if y == 1<<10 {  // << means to the power
            break; // telling the program to break out of the loop when if statement is true
        }
    }
}
