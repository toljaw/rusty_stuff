// Arrays: Fixed length, only same data types

fn main() {
    let _array = [1, 2, 3];

    println!("{_array:?}");
    println!("{_array:#?}");        // # for pretty printing
    println!("{}", _array[0]);      // output single array content
    println!("{}", _array[1]);
    println!("{}", _array[2]);

    let _array2: [i32; 5] = [1337; 5];

    println!("{:?}", _array2);
    println!("{}", _array2[0]);
    println!("{}", _array2[1]);
    println!("{}", _array2[2]);
    println!("{}", _array2[3]);
    // println!("{}", _array2[5]);      // 5 is out of bounce - compiler will also complain
}