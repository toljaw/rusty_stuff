const MILLISECONDS_PER_SECONDS:i32 = 1000;      // const benutzt man bei globalen Variablen, außerdem sind sie performanter als Mutables
const MICROSECONDS_PER_SECONDS:i32 = MILLISECONDS_PER_SECONDS * 1000;   // bei const muss typ immer mit angegeben werden
const NANOSECONDS_PER_SECONDS:i32 = MICROSECONDS_PER_SECONDS * 1000;

fn main() {
    println!("{MILLISECONDS_PER_SECONDS}");
    println!("{MICROSECONDS_PER_SECONDS}");
    println!("{NANOSECONDS_PER_SECONDS}");
}