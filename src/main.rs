

use std::mem;
enum Color {
    RED,
    GREEN,
    BLUE,
    RgbColor(u8,u8,u8), //tuple structure object
    CmykColor{ //struct object that holds 4 u8 date type objects
        cyan:u8,
        magenta:u8,
        yellow:u8,
        black:u8
    },

}


fn enums(){
    let c = Color::CmykColor {cyan:0, magenta:128, yellow:0, black:255};

    match c { //match is like a switch case statement
        Color::RED => println!("red"),
        Color::GREEN => println!("green"),
        Color::BLUE => println!("blue"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: 128,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{}", r, g, b),
        _ => (),

    };
}

fn main() {
    enums();
}

