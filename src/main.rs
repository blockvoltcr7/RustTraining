use std::collections::HashMap;

fn main() {

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (key,value)  in &shapes {

        println!("{} : {}",key,value)
    }

    println!("{:?}", shapes);

    //if cicle doesnt exist then add it to the hashmap
    shapes.entry("cicle".into()).or_insert(1);
    println!("{:?}", shapes);

}

