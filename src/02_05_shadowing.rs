fn main() {
    // Example 1
    {
        // shadowing means, that you can assign a variable inner scope, you assigned outer scope already
        // it can be used inner scope only
        let x = 1337;   // 0xAA (exemplarischer Speicher)
        println!("{x}");

        {
            let x = -10;
            println!("{x}");
        }

        println!("{x}");
        println!();
    }

    // Example 2
    {
        // if variable is mutable, it will be overwritten everytime (without "let")
        let mut y = 1337;   // 0xAA
        println!("{y}");

        y = 0;                  // 0xAA
        println!("{y}");

        y = 10;
        println!("{y}");
        println!();
    }

    // Example 3
    {
        // if you assign a variable with "let" again, then the memory will be not overwritten
        // and the second assign will be used only once - >>once probably means in local scope only<<
        // after that the first variable will be used again
        let z = 1337;       // 0xAA
        println!("{z}");

        let z = 0;          // 0xBB
        println!("{z}");

        let y = 10;
        let yz = y+z;
        println!("{yz}");
    }
}