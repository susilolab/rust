use std::io;

struct Menu {
	list_menu: Vec<String>,
	answer: i32,
}

impl Menu {
	fn show_menu(&mut self) {
		print!("{}[2J", 27 as char);
		
		for menu in &self.list_menu {
			println!("{}", menu);
		}
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		if let Ok(answer) = input.trim().parse::<i32>() {
			self.answer = answer;
		} else {
			self.answer = 0;
		}
	}

	fn run(&mut self) {
		while self.answer != 4 {
			self.show_menu();
		}
	}
}

fn main() {
	let list_menu = vec![
		"1. User".to_owned(),
		"2. Menu".to_owned(),
		"3. Help".to_owned(),
		"4. Quit".to_owned()
	];

	let mut menu: Menu = Menu{
		list_menu: list_menu,
		answer: 0,
	};

	menu.run();
}
