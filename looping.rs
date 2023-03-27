fn main() {
    let mut count = 0;

    loop {
        count += 1;

        //don't let this run infinitely
        if count > 100 {
            println!("We need to break the cycle sometime!!");
            break;
        }
    }

    for i in 1..100 {
        if i == 99 {
            println!("{} iterations. Done.", i);
        }
    }

    let v = vec!["horse", "sheep", "dog", "pig"];
    for animal in v {
        println!("{} is a farm animal", animal);
    }
}
