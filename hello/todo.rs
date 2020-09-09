struct Todo {
	id: i32,
	body: String,
}

trait Crud {
	fn add(&self, id: i32, body: String);
	fn list(&self);
}

struct Todos {
	todo: Vec<Todo>,
}

impl Crud for Todos {
	fn add(&self, id: i32, body: String) {
		self.todo.push(Todo { id: id, body: body });
	}

	fn list(&self) {
		for todo in self.todo {
			println!("id: {}, body: {}", todo.id, todo.body);
		}
	}
}

fn main() {
	let todos: Todos = Todos { todo: Vec::new(), };
}
