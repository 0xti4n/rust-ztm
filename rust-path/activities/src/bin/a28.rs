// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[derive(Debug)]
struct Shoe(Color);
#[derive(Debug)]
struct Shirt(Color);
#[derive(Debug)]
struct Pants(Color);

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

impl Shoe {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoe_color(color: Shoe) {
    println!("{:?}", color);
}

fn print_shirt_color(color: Shirt) {
    println!("{:?}", color);
}

fn print_pants_color(color: Pants) {
    println!("{:?}", color);
}

fn main() {
    let pants = Pants::new(Color::Custom("amarillo".to_owned()));
    let shoe = Shoe::new(Color::Black);
    let shirt = Shirt::new(Color::Black);

    print_pants_color(pants);
    print_shirt_color(shirt);
    print_shoe_color(shoe);
}
