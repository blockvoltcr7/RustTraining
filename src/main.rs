fn main() {


    //enhanced string for loop

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"))
    }
    println!("passed")

}
