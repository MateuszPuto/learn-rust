fn main() {
    #[derive(Debug)]
    struct Car {
        make: String,
        model: String,
        year: u32,
        max_speed: i32,
    }

    impl Car {
        fn compare_speed(&self, other: &Car) {
            let margin: i32 = 5;

            if self.max_speed - other.max_speed > margin {
                println!(
                    "Yours car - {} {} {} is faster than {} {} {}! 🏎️.",
                    self.make, self.model, self.year, other.make, other.model, other.year
                );
            } else if self.max_speed - other.max_speed > 0 {
                println!(
                    "Equal max speed between {} {} {} and {} {} {}.",
                    self.make, self.model, self.year, other.make, other.model, other.year
                );
            } else {
                println!(
                    "Yours car - {} {} {} is slower than {} {} {}. :(",
                    self.make, self.model, self.year, other.make, other.model, other.year
                );
            }
        }
    }

    let corolla = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2022,
        max_speed: 190,
    };
    let porsche911 = Car {
        make: String::from("Porsche"),
        model: String::from("911 GT3"),
        year: 2022,
        max_speed: 319,
    };
    let older_porsche = Car {
        make: String::from("Porsche"),
        model: String::from("911 GT3"),
        year: 2021,
        max_speed: 317,
    };

    porsche911.compare_speed(&corolla);
    porsche911.compare_speed(&older_porsche);
    corolla.compare_speed(&porsche911);
}
