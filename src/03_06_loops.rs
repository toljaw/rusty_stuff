fn call_loop() {
    let mut counter = 0;

    'looper_name: loop {            // loops can be named - makes only sense if you have more inner
        // loops and want to specify the loop you want to break for example
        counter += 1;
        println!("{}", counter);

        if counter == 10 {
            break 'looper_name;
        }
    }
}

fn call_while() {
    let mut counter = 0;

    'whiler_name: while counter < 10 {
        counter += 1;
        println!("{}", counter);
    }
}

fn call_for() {
    // [0, ..., 10]
    for num in 0..=11 {          // ...in 0..=11 is including 11
        println!("{}", num);
    }
}


fn main() {
    call_loop();
    println!();

    call_while();
    println!();

    call_for();
}