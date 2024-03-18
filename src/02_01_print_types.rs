fn main() {
    let s = "Hello World";
    print!("{}",s);             // print druckt ohne Zeilenumbruch
    print!("{}\n",s);           // \n druckt mit Zeilenumbruch
    println!("{}", s);          // mit Zeilenumbruch

    let name = "Anatoli";
    let s2 = format!("Hello {}, nice to meet you!", name);
    println!("{}", s2);
    // seit Rust 1.58 auch:
    let s3 = format!("Hello {name}, nice to meet you!");
    println!("{}", s3);
    println!("{s3}");
}
