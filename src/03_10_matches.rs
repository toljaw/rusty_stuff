fn main() {
    let number = 1;

    match number {
        1 => {                                          // {} sind optional - gut für mehrzeilige
            // Eingaben
            println!("Just...");
            println!("One...");
        },
        2 | 3 | 5 | 7 | 11 => println!("Prime..."),     // mehrere Werte mit "|" aufzählen
        _ => println!("None!..."),                      // "_" means default
    }

    println!();

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}