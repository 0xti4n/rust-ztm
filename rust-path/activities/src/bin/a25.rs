// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calc_perimeter(&self) -> i32;
}

struct Square {
    x: i32,
}
impl Perimeter for Square {
    fn calc_perimeter(&self) -> i32 {
        self.x * 4
    }
}

struct Triangle {
    left: i32,
    right: i32,
    bottom: i32,
}
impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> i32 {
        self.left + self.right + self.bottom
    }
}

fn calculate(figure: impl Perimeter) {
    let shape = figure.calc_perimeter();
    println!("the perimeter is {:?}", shape);
}

fn main() {
    calculate(Square { x: 5 });
    calculate(Triangle {
        left: 4,
        right: 4,
        bottom: 4,
    });
}
