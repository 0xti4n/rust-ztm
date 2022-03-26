// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if a == 0 {
        return None;
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn division() {
        let a: i32 = 0;
        let b: i32 = 4;
        let result = div(a, b);

        assert_eq!(result, None);
    }

    #[test]
    fn clamp_test() {
        let n = 4;
        let lower = 5;
        let upper = 6;

        assert_eq!(clamp(n, lower, upper), lower);
        assert_ne!(clamp(n, lower, upper), upper);
    }

    #[test]
    fn concat_test() {
        let first = "hello";
        let second = "world";

        assert_eq!(concat(&first, &second), "helloworld");
    }
}
