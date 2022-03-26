// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage<State> {
    tracking_id: i32,
    state: State,
}

impl<State> Luggage<State> {
    fn transition<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage {
            tracking_id: self.tracking_id,
            state: state,
        }
    }
}

struct CheckIn;
struct OnLoading;
struct OffLoading;
struct AwaitingPickup;
struct EndCustody;

impl Luggage<CheckIn> {
    fn new(tracking_id: i32) -> Self {
        Self {
            tracking_id: tracking_id,
            state: CheckIn,
        }
    }

    fn check_in(self) -> Luggage<OnLoading> {
        self.transition(OnLoading)
    }
}

impl Luggage<OnLoading> {
    fn transfer_plane(self) -> Luggage<OffLoading> {
        self.transition(OffLoading)
    }
}

impl Luggage<OffLoading> {
    fn taken_off_plane(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    fn await_pickup(self) -> Luggage<EndCustody> {
        self.transition(EndCustody)
    }
}

fn main() {
    let luggage = Luggage::new(17);

    let tracking = luggage
        .check_in()
        .transfer_plane()
        .taken_off_plane()
        .await_pickup();

    match tracking.state {
        EndCustody => println!(
            "your luggage id: {} was pickup successfully",
            tracking.tracking_id
        ),
        //_ => println!("something was wrong with your luggage")
    }
}
