fn number_checker(number:i32) {
    if number % 4 == 0 {                        // "number % 4 == 0" muss hier nicht in Klammern
        // gesetzt werden
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn main() {
    let number = 7;
    number_checker(number);         // Ausführung der zuvor definierten Funktion

    //ternary operator: <bedingung> ? <wert_wenn_wahr> : <wert_wenn_falsch>
    let result = if number > 5 {"größer als 5"} else {"kleiner oder gleich 5"};

    println!("Das Ergebnis ist: {}", result);


}