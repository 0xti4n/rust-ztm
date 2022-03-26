// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

// * Use an enum to represent all types of employees
enum Position {
    MaintenanceCrews,
    MarketingDepartment,
    Managers,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum Status {
    Active,
    NotActive,
}

// * Use a struct to store the employee type and whether they are
//   still employed
struct Employe {
    position: Position,
    status: Status,
}

// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn get_access(employe: &Employe) -> Result<(), String> {
    match employe.status {
        Status::NotActive => return Err("employe not active".to_owned()),
        _ => (),
    }

    match employe.position {
        Position::Managers => Ok(()),
        Position::MarketingDepartment => Ok(()),
        Position::MaintenanceCrews => Ok(()),
        _ => Err("access invalid".to_owned()),
    }
}

// * Print whether the employee may access the building

//   * Must use a function that utilizes the question mark operator to do this
fn check_access(employe: &Employe) -> Result<(), String> {
    get_access(&employe)?;
    println!("Access ok");
    Ok(())
}

fn main() {
    let employe = Employe {
        position: Position::Managers,
        status: Status::Active,
    };

    match check_access(&employe) {
        Err(e) => println!("acces denied {:?}", e),
        _ => (),
    }
}
