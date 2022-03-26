// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use colored::*;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: i32,
}

impl Bill {
    fn new(bill_name: &str, amount: i32) -> Self {
        Self {
            name: bill_name.to_owned(),
            amount: amount.to_owned(),
        }
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let mut bills = HashMap::new();
    loop {
        println!("============================");
        println!("   🤖 {} {}  🤖", "Manage".yellow(), "Bills".yellow());
        println!("============================\n");
        println!("{}", "1. Add bill".blue());
        println!("{}", "2. View bills".blue());
        println!("{}", "3. Remove bill".blue());
        println!("{}", "4. Update bill".blue());
        println!("{}", "5. Bill total".blue());
        println!();
        println!("=> {}", "Select a Number".green());

        match get_input().trim().to_owned().as_str() {
            "1" => {
                println!("=> {}", "Enter Name of Bill:".green());
                let name = get_input().trim().to_owned().to_lowercase();

                if bills.contains_key(&name) {
                    println!("\n{:?} Already exist!\n", name);
                    continue;
                }

                println!("=> {}", "Enter Amount:".green());
                let amount: i32 = match get_input().trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}", "Error: (amount must be a number)\n".red());
                        continue;
                    }
                };

                let data = Bill::new(&name, amount);
                bills.insert(data.name, data.amount);
                println!("\n💸 {} 💸\n", "--Bill Saved!--".red());
            }
            "2" => {
                if bills.is_empty() {
                    println!("\nYou don't have Bills saved!\n");
                } else {
                    println!("\nBills Info:\n ");
                    for (k, v) in &bills {
                        println!("{} Name: {:?}  Amount: ${:?}\n", '🪙', k, v);
                    }
                }
            }
            "3" => {
                println!("\nEnter Name of Bill to Remove:");
                let name_bill = get_input().trim().to_owned().to_lowercase();

                match &bills.remove(&name_bill) {
                    Some(key) => println!(
                        "\nBill Remove Succes! (Name: {:?} - Amount: ${})\n",
                        name_bill, key
                    ),
                    None => {
                        println!("\nError: Bill name not found!\n");
                        continue;
                    }
                }
            }
            "4" => {
                println!("\nEnter Name of Bill to Update:");
                let name_bill = get_input().trim().to_owned().to_lowercase();

                if bills.contains_key(&name_bill) {
                    println!("\nEnter Amount to Update:");

                    let amount: i32 = match get_input().trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("{}", "Error: (amount must be a number)\n".red());
                            continue;
                        }
                    };
                    bills.insert(name_bill, amount);
                    println!("\nBill Update Success!\n");
                } else {
                    println!("\n{:?} Bill name not exist!\n", name_bill);
                    continue;
                }
            }
            "5" => {
                println!("\nYou Have a Total of ({}) Bills!\n", bills.len());
            }

            _ => println!("Invalid option"),
        }
    }
}
