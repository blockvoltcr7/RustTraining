
fn main() {


    vectors();

}

fn vectors(){

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    println!("a[0] = {:?}", a[0]);


    for x in &a {
        println!("{}",x)
    }

    //remove
    let last_elem = a.pop();
    println!("{:?}",last_elem)
}