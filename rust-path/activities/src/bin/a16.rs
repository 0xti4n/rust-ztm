// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_number: Option<i32>,
}

fn main() {
    let student = Student {
        name: String::from("s1"),
        locker_number: Some(44),
    };

    println!("{:?}", student.name);

    match student.locker_number {
        Some(number) => println!("{:?}", number),
        None => println!("no locker number found"),
    }
}
