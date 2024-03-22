// enum Option<T> {
//     Some(T),
//     Nome,
// }

fn main() {
    let x:u32 = 5;
    let y:Option<u32> = Some(7);

    println!("x + y = {}", add(x,y));
}

fn add(x:u32, y: Option<u32>) -> u32 {
    match y {
        Some(val) => x + val,
        None => x + 0,
    }
}