fn main() {
    let mut hello = String::from("Rust ❤️");    // chars can have 1 - 4 bytes because os UTF 8
    // support - emojis can have 4 bytes and letters 1 byte - also can not call chars by index
    // because this reason

    hello.push('!');            // push funktioniert nur wenn variable mutable ist
    hello.push('!');
    hello.push('!');

    println!("{}", hello);

    for b in hello.bytes() {        // .bytes gibt bytes wieder - bei einem character mit
        // mehreren bytes werden alle bytes einzeln wiedergegeben
        println!("{}", b);
    }
    println!();
    for c in hello.chars() {        // . chars gibt characters wieder
        println!("{}", c);
    }

}