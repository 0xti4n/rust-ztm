// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Costumer {
    age: i32,
}

fn check_purchase(costumer: Costumer) -> Result<String, String> {
    match costumer.age {
        21 => Ok("can purchase".to_owned()),
        _ => Err("can't purchase".to_owned()),
    }
}

fn main() {
    let user = Costumer { age: 20 };

    let check = check_purchase(user);
    println!("{:?}", check);
}
