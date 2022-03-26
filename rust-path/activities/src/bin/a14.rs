// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

fn print_person(name: &str, fav_color: &str) {
    print!("name: {} ", name);
    println!("fav_color: {}", fav_color);
}

fn main() {
    let persons = vec![
        Person {
            age: 10,
            name: String::from("p1"),
            fav_color: "red".to_owned(),
        },
        Person {
            age: 15,
            name: String::from("p2"),
            fav_color: "green".to_owned(),
        },
        Person {
            age: 5,
            name: String::from("p3"),
            fav_color: "blue".to_owned(),
        },
    ];
    for p in persons {
        if p.age <= 10 {
            print_person(&p.name, &p.fav_color);
        }
    }
}
