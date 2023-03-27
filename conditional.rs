fn main() {
    let x: i32 = 42;

    if x > 50 {
        println!("Big number! ({})", x);
    } else if x == 42 {
        println!("{} - The answer to life, universe and evertyhing.", x);
    } else {
        println!("Just some small number.");
    }
}
