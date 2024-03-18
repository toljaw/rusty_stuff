fn main() {
    let _x1 = 0;                        // default 32bit signed (mit Vorzeichen) integer
    let _x2:i8 = 0;                      // i = integer - 8 bit signed (mit Vorzeichen) integer
    let _x3:i16 = 0;
    let _x4:i32 = 0;
    let _x5:i64 = 0;
    let _x6:i128 = i128::MIN;           // if sign a variable with "_" the compiler will not complain and show issues

    let _x7:u8 = u8::MAX;               // u = unsigned integer // u8::MAX = max value
    let _x8:u16 = u16::MAX;
    let _x9:u32 = u32::MAX;
    let _x10:u64 = u64::MAX;
    let _x11:u128 = u128::MAX;
    let _x12:u128 = u128::MIN;

    println!("{_x6}, {_x7}, {_x11}, {_x12}");

    let _dec = 255;                     // Zuweisung eines Dezimal Wertes
    println!("{_dec}");
    let _hex = 0xff;                    // Zuweisung eines Hexadezimal Wertes
    println!("{_hex}");
    let _bin = 0b11111111;              // Zuweisung eines Binärwertes
    println!("{_bin}");

    let _f1 = 2.0;                      // default 64 bit floating point
    let _f2:f32 = 2.0;                  // same as "let _f2 = 2.0_f32;" - diese Schreibweise ist nur für nummerische Datentypen verfügbar
    let _f3 = 3.0_f64;                  // same as "let _f3:f64 = 3.0;"

    let _flag1 = true;
    let _flag2 = false;
}