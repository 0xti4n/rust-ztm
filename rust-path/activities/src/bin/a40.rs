// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Available,
    Unavailable,
    Rented,
    Maintenance,
}

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug)]
struct Rental {
    status: Status,
    vehicle: Vehicle,
    vin: String,
}

type data = Rc<RefCell<Vec<Rental>>>;

struct Corporate(data);

struct StoreFront(data);

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let vehicles = vec![
            Rental {
                status: Status::Available,
                vehicle: Vehicle::Car,
                vin: "123".to_owned(),
            },
            Rental {
                status: Status::Unavailable,
                vehicle: Vehicle::Truck,
                vin: "adc".to_owned(),
            },
        ];

        let vehicles = Rc::new(RefCell::new(vehicles));
        let corporate = Corporate(Rc::clone(&vehicles));
        let storefront = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = storefront.0.borrow_mut();
            let car = match rentals.get_mut(0) {
                Some(car) => {
                    assert_eq!(car.status, Status::Available);
                    car.status = Status::Rented
                }
                None => (),
            };
        }

        {
            let mut rentals = corporate.0.borrow_mut();
            let car = match rentals.get_mut(0) {
                Some(car) => {
                    assert_eq!(car.status, Status::Rented);
                    car.status = Status::Available
                }
                None => (),
            };
        }

        let rentals = corporate.0.borrow();
        match rentals.get(0) {
            Some(car) => assert_eq!(car.status, Status::Available),
            None => (),
        }
    }
}
