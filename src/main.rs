


fn useSlice(slice: &mut[i32]){//borrowing a slice of an array of i32

    println!("first elem = {}, len = {}", slice[0] , slice.len());
    slice[0] = 4321;
}

fn main() {


    let mut data = [1,2,3,4,5];


    useSlice(&mut data[1..4]); //we are taking index 1-3 and not index 0
    useSlice(&mut data);
    println!("{:?}", data);

}

