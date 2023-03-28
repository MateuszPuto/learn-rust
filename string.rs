fn main() {
    let names = [
        String::from("Программирование"),
        String::from("程序设计"),
        String::from("ﺏﺮﻤﺟﺓ"),
    ];
    let langs = ["Russian", "Chinese", "Arabic"];
    let mut sents = Vec::new();

    for (i, language) in langs.iter().enumerate() {
        let sentence = format!("{} means Programming in {}", names[i], language);

        println!("{}", sentence);
        spelling(&names[i]);

        sents.push(sentence);
    }
}

fn spelling(name: &str) {
    println!("Word is spelled as:");
    for c in name.chars() {
        println!("Next char: {}", c);
    }
}
