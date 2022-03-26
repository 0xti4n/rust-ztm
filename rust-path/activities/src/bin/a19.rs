// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();

    store.insert("Chairs", 5);
    store.insert("Beds", 3);
    store.insert("Tables", 2);
    store.insert("Couches", 0);

    let mut total_store = 0;

    for (k, v) in store.iter() {
        total_store += v;
        if v == &0 {
            println!("{}: Out of stock", k);
        } else {
            println!("{}: {}", k, v);
        }
    }
    println!("total {}", total_store);
}
