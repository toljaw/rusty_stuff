// bei Rust kann eine Funktion aufgerufen werden, bevor sie definiert wurde

fn main() {
    let num = 4;

    let ret = my_function(num);

    println!("{}", ret);
}

// bei Funktionen müssen Parameter und Return Werte type annotiert werden
fn my_function(input:i32) -> i32 {
    input * 2                       // same as "return input * 2;" - in Rust just miss return and
    // semicolon for return
}
