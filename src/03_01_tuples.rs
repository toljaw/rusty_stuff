// Tuples: Fixed length, different data types possible

fn main() {
    let mut tpl = (500, "hi", true);

    //println!("{tpl}");    // std::fmt::Display
    println!("{tpl:?}");    // std::fmt::Debug

    let (x, y, z) = tpl;            // tuples can be unpacked - the new variables are copies and can be changed if mutable
    println!("{x}");
    println!("{y}");
    println!("{z}");

    tpl.2 = false;                                  // tuple content can be updated - here index 2
    println!("{tpl:?}");
}