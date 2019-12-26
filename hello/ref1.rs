/// Demo reference/ borrowing vector
/// 
fn main() {
	// fungsi sum_vec meminjam var v dengan tipe &Vec<i32>
	fn sum_vec(v: &Vec<i32>) -> i32 {
		return v.iter().fold(0, |a, &b| a + b);
	}

	// fungsi foo meminjam var v1 dan v2
	fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
		let s1 = sum_vec(v1);
		let s2 = sum_vec(v2);
		return s1 + s2;
	}

	let v1 = vec![1, 2, 3];
	let v2 = vec![4, 5, 6];
	let jawab = foo(&v1, &v2);
	println!("{}", jawab);
}
