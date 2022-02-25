
fn main() {


    let country_code = 1001;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);

    let x = true;

    //match is like an if, but the cool thing is that it can test a range of cases, almost like a switch case statement
    let s = match x {
        true => "yes",
        false => "no"
    };

    println!("{}",x)

}
