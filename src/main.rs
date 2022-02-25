
fn main() {

    for x  in 0..11
    {
        if x == 3 {
            continue;
        }

        if x == 8 {
            break;
        }

        println!("{}", x)

    }


    //give me range from 30 and 40
    for (position, y) in (0..41).enumerate(){
        println!("{}: {}",position,y);
    }


}
