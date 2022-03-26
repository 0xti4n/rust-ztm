// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;

enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn convert_input(new_input: &str) -> Result<PowerStates, String> {
    match new_input {
        "off" => Ok(PowerStates::Off),
        "sleep" => Ok(PowerStates::Sleep),
        "reboot" => Ok(PowerStates::Reboot),
        "shutdown" => Ok(PowerStates::Shutdown),
        "hibernate" => Ok(PowerStates::Hibernate),
        _ => Err("commnad not found".to_owned()),
    }
}

fn print_input(input: PowerStates) {
    use PowerStates::*;

    match input {
        Off => println!("power off"),
        Sleep => println!("Sleeping"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn get_input() -> io::Result<String> {
    println!("Enter a Power state: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    match get_input() {
        Ok(input) => {
            let data = convert_input(&input.to_lowercase());

            match data {
                Ok(input) => print_input(input),
                Err(e) => println!("error: {:?}", e),
            }
        }
        Err(e) => println!("error {:?}", e),
    }
}
