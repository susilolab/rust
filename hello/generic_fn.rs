struct Print;

trait Printer<T> {
    fn print_vec(data: T);
}

impl Printer<Vec<i32>> for Print {
    fn print_vec(data: Vec<i32>) {
        for i in data {
            println!("{}", i);
        }
    }
}

impl Printer<Vec<&str>> for Print {
    fn print_vec(data: Vec<&str>) {
        for i in data {
            println!("{}", i);
        }
    }
}

fn main() {
	let v = vec![1, 2, 3];
	// print_vec(v);
    Print::print_vec(v);

	let str_v = vec!["Hello", "you", "world"];
	// print_vec(str_v);
    Print::print_vec(str_v);
}
