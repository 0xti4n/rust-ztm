// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn cord_tuple() -> (i32, i32) {
    (3, 1)
}

fn main() {
    let (_, y) = cord_tuple();

    if y > 5 {
        println!("greather than 5");
    } else if y < 5 {
        println!("lesser than 5");
    } else {
        println!("equal to 5");
    }
}
