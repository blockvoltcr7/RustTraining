
fn main() {


    let temp: u8 = 5;

    if temp > 25 {
        println!("true temp is greater")
    } else if temp < 25 {
        println!("true temp is {} which is less than 25", temp)
    }else{
        println!("temp is neutral")
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("{}",day)



}
