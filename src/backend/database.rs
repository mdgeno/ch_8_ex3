use std::io;
use std::collections::HashMap;

pub struct EmployeeDatabase{
	map: HashMap<String, String>
}

impl EmployeeDatabase{
	pub fn new_list() -> EmployeeDatabase{
		EmployeeDatabase{ map: HashMap::new() }
	}

	pub fn add_employee(&mut self, k: &str, v: &str){

/* //don't forget to remove parameters
		println!("Enter Full Name");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter correct input");
		let key_name = input.trim();

		println!("Enter Department");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("enter correct input");
		let value_dep = input.trim();

		self.map.insert(key_name.to_string(), value_dep.to_string());
*/
	self.map.insert(k.to_string(), v.to_string());

	}

	
	pub fn print_all_staff(&self){
		let mut all_staff = Vec::new();
		for k in self.map.keys(){
			all_staff.push(k);
		};
		all_staff.sort();
		for staff in &all_staff{
			println!("{staff}");
		}
	}

	pub fn print_department(&self, dep: &str){
		let mut all_in_dep = Vec::new();
		for k in self.map.keys(){
			let mut department = match self.map.get(k){
						Some(val) => val,
						None => continue
					     };
			if department == dep {
				all_in_dep.push(k);
			}else{
				continue;
			}
		}

		for staff in all_in_dep{
			println!("{staff}");
		}
	}
	
}