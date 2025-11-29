use crate::backend::database::EmployeeDatabase;
use std::io;

pub mod backend;

fn main() {

	let mut list = EmployeeDatabase::new_list();
/*	
	loop{
		println!("Enter number of desired action:");
		println!("1 Add Employee");
		println!("2 List all Employees");
		println!("3 Exit Program");

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter correct input");

		match input.trim(){
			"1" => {list.add_employee();
				continue},
			"2" => {list.print_all_staff();
				break},
			"3" => break,
			_ => {println!("incorrect input");
			      continue}
		}
	}
*/

	let emp1 = String::from("EmployeeA One");
	let emp2 = String::from("EmployeeB Two");
	let emp3 = String::from("EmployeeC Three");
	let emp4 = String::from("EmployeeD Four");
	let emp5 = String::from("EmployeeE Five");
	let emp6 = String::from("EmployeeF Six");	

	let engineering = String::from("Engineering");
	let sales = String::from("Sales");

	list.add_employee(&emp1, &engineering);
	list.add_employee(&emp5, &sales);
	list.add_employee(&emp6, &sales);
	list.add_employee(&emp4, &sales);
	list.add_employee(&emp2, &engineering);
	list.add_employee(&emp3, &engineering);

	list.print_all_staff();

	println!(" ");
	println!("print all staff in Sales department");

	list.print_department("Sales");
}
