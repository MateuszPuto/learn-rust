fn main() {
    enum Flavour {
        Chocolate,
        Vanilla,
        Strawberry,
        Mint,
        Cookie,
        Caramel,
        Lemon,
    }

    fn make_ice_cream(flavour: Flavour, scoops: u8) -> String {
        match flavour {
            Flavour::Chocolate => {
                format!("Great choice! {} scoop(s) of chocolate ice cream.", scoops)
            }
            Flavour::Vanilla => {
                format!("Classic! {} scoop(s) of vanilla ice cream for you.", scoops)
            }
            Flavour::Strawberry => format!(
                "Fruit flavour day I assume. Here you go, {} scoop(s) of strawberry ice cream.",
                scoops
            ),
            _ => format!("Sorry. We don't sell those here."),
        }
    }

    let orders = vec![
        (Flavour::Chocolate, 2),
        (Flavour::Vanilla, 1),
        (Flavour::Mint, 3),
        (Flavour::Strawberry, 2),
        (Flavour::Cookie, 1),
        (Flavour::Caramel, 5),
        (Flavour::Lemon, 2),
    ];

    for order in orders {
        let (flavour, no_scoops) = order;
        println!("{}", make_ice_cream(flavour, no_scoops));
    }
}
