use std::io;
use std::collections::HashMap;

pub struct EmployeeDatabase{
	map: HashMap<String, String>
}

impl EmployeeDatabase{
	pub fn new_list() -> EmployeeDatabase{
		EmployeeDatabase{ map: HashMap::new() }
	}

	pub fn add_employee(&mut self){ //k: &str, v: &str){
		println!("Enter Full Name");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter correct input");
		let key_name = input.trim();

		println!("Enter Department");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter correct input");
		let value_dep = input.trim();

		self.map.insert(key_name.to_string(), value_dep.to_string());
		
		println!(" ");
/*
		self.map.insert(k.to_string(), v.to_string());
*/

	}

	
	pub fn print_all_staff(&self){
		println!(" ");

		let mut all_staff = Vec::new();
		for k in self.map.keys(){
			all_staff.push(k);
		};
		all_staff.sort();
		for staff in &all_staff{
			println!("{staff}");
		}

		println!(" ");
	}

	pub fn print_dep(&self){
		println!(" ");
		println!("Existing Departments:");
		self.all_dep();
		println!("Input the Department you want to view:");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Enter correct input");

		let mut all_in_dep = Vec::new();
		for k in self.map.keys(){
			let department = match self.map.get(k){
						Some(val) => val,
						None => continue
					     };
			if department == input.trim() {
				all_in_dep.push(k);
			}else{
				continue;
			}
		}

		println!(" ");
		println!("All Employees in {} Department", input.trim());	
		all_in_dep.sort();
		for staff in all_in_dep{
			println!("{staff}");
		}
		println!(" ");
	}

	fn all_dep(&self){
		let mut dep = Vec::new();
		for department in self.map.values(){
			if !dep.contains(&department){
				dep.push(&department);
			}	
		}

		for department in dep{
			println!("{department}");
		}
	}	
}

























