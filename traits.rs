pub mod movie {
    pub struct Animal {
        pub name: &'static str,
        pub sound: &'static str,
    }

    pub struct Character {
        pub name: &'static str,
        pub common_saying: &'static str,
    }

    pub trait Speak {
        fn speak_line(&self);
    }

    impl Speak for Animal {
        fn speak_line(&self) {
            println!("\tANIMAL\n{}\n", self.sound);
        }
    }

    impl Speak for Character {
        fn speak_line(&self) {
          println!("\t{}\n{}\n", self.name.to_uppercase(), self.common_saying);  
        }
    }

}

use movie::Animal;
use movie::Character;
use movie::Speak;

fn main() {
    let dog = Animal{name: "Fluffy", sound: "bark, bark"};
    let cat = Animal{name: "Softball", sound: "meow meow"};
    let john = Character{name: "Dr John", common_saying: "Wait here little bunny I'm gonna get your cure soon."};

    dog.speak_line();
    cat.speak_line();
    john.speak_line();
}
