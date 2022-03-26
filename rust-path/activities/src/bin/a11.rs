// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    quantity: i32,
    number: i32,
}

fn display_qty(item: &Item) {
    println!("{:?}", item.quantity);
}

fn display_id(item: &Item) {
    println!("{:?}", item.number);
}

fn main() {
    let item = Item {
        quantity: 30,
        number: 1,
    };

    display_qty(&item);
    display_id(&item);
}
