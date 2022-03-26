// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("color: red"),
            Color::Blue => println!("color: blue"),
        }
    }
}

struct Dimensions {
    w: f64,
    h: f64,
    d: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("w: {}", self.w);
        println!("h: {}", self.h);
        println!("d: {}", self.d);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_box(&self) {
        self.dimensions.print();
        self.color.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let dm = Dimensions {
        w: 3.0,
        h: 5.0,
        d: 5.0,
    };

    let box1 = ShippingBox::new(5.0, Color::Red, dm);
    box1.print_box();
}
