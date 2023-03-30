fn main() {
    let sent = String::from(" May the Force be with you ");

    match sent.to_lowercase() {
        sent if sent.contains(" i ") | sent.contains(" we ") | sent.contains(" us ") => {
            println!("First person pronoun.")
        }
        sent if sent.contains(" you ") => println!("Second person pronoun."),
        sent if sent.contains(" he ")
            | sent.contains(" she ")
            | sent.contains(" it ")
            | sent.contains(" they ") =>
        {
            println!("Third person pronoun.")
        }
        _ => println!("Not talking about people."),
    }
}
