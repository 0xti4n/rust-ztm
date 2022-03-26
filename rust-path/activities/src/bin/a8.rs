// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Banana,
    Strawberry,
}

struct Drink {
    flavor: Flavor,
    ounces: i32,
}

fn prt(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("orange"),
        Flavor::Banana => println!("Banana"),
        Flavor::Strawberry => println!("Strawberry"),
    }
    println!("{:?}", drink.ounces)
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Orange,
        ounces: 10,
    };

    let drink1 = Drink {
        flavor: Flavor::Banana,
        ounces: 30,
    };

    let drink2 = Drink {
        flavor: Flavor::Strawberry,
        ..drink
    };

    prt(drink);
    prt(drink1);
    prt(drink2);
}
