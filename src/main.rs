use std::collections::HashSet;

fn main() {


    let mut countries = HashSet::new();

    //hashset only stores unique elements
    countries.insert("morocco");
    countries.insert("slovakia");
    countries.insert("slovakia");

    println!("{:?}", countries);

    let added = countries.insert("brazil");

    if added {
        println!("{}", added);
    }

    //check if hashset has a value

    if !countries.contains("brazil") {
        println!("we dont have brazil");
    }else{
        println!("we do have brazil");

    }


    //remove an element from hashset
    let removed = countries.remove("brazil");
    if removed {
        println!("we removed brazil")
    }


    let _1_5: HashSet<_> =  ( 1..=5).collect();
    let _6_10: HashSet<_> =  ( 6..=10).collect();
    let _1_10: HashSet<_> =  ( 1..=10).collect();
    let _2_8: HashSet<_> =  ( 2..=8).collect();

    //subset, every element is contain in the other set
    println!("is {:?} a subset of {:?} ? {}", _2_8, _1_10, _2_8.is_subset(&_1_10))

}

