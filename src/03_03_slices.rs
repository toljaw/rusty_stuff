use std::mem;

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    println!("length: {}", xs.len());
    println!("data size: {}", mem::size_of_val(&xs));       // will output 20 - 5 * 4 byte (4 * 8 bit = 32 bit = 4 byte)

    println!("Slice: {:?}", &xs[1..4]);                     // first value included - last value not included
    println!("Slice: {:?}", &xs[1..=4]);                     // if need to include last value

}