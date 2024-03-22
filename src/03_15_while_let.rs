use std::array::IntoIter;

fn while_letter1() {            // mit einer Range
    let mut nums = 0..=11;

    // while (Some(num) == nums.next()) - nach dem letzten num kommt ein None
    while let Some(num) = nums.next() {         // ähnlich einer For Loop
        println!("{num}");
    }
}

fn while_letter2() {            // mit einem Array
    let mut nums = [1, 2, 3].into_iter();

    while let Some(num) = nums.next() {
        println!("{}",num);
    }
}

fn main() {
    while_letter1();
    println!();
    while_letter2()
}