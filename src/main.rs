


use std::io::stdin;

enum State {
    LOCKED,
    FAILED,
    UNLOCKED
}

fn main() {

    let code = String::from("1234");
    let mut state = State::LOCKED;
    let mut entry = String::new();


    //loop state
    loop {

        //match state
        match state {
            //start the state as locked
            State::LOCKED => {
                let mut input = String::new(); //create a new string for the input
                match stdin().read_line(&mut input){ //take stdin entry from console and set as input
                    Ok(_) => {
                        entry.push_str(&input.trim_end()); //entry 1,2 etc appending value to the string

                    }
                    Err(_) => continue
                }

                //check if entry matches the code, if true then state is unlocked
                if entry == code {
                    state = State::UNLOCKED;
                    continue;
                }

                //check if code starts with something that is not valid
                if !code.starts_with(&entry){
                    //entry is 1234 but entered 125 then fail
                    state = State::FAILED;
                }
            }

            //failed case
            State::FAILED => {
                println!("FAILED!");
                entry.clear();
                state = State::LOCKED;
                continue;
            }

            //success case
            State::UNLOCKED => {
                println!("UNLOCKED!");
                return;
            }

        }
    }


}
