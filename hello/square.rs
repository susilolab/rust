/// Mencari total hasil dari 2 pangkat 1000 => 2^1000
///
/// 2^1000 = ?
/// ? dicari total dari penjumlahan angka tsb.
///
fn main() {
    let num: f64 = 2.0;
    let power: String = num.powi(1000).to_string();
    let vec_num: Vec<&str> = power.split("").collect();

    let sum: i32 = remove_non_number(&vec_num).into_iter().sum();
    println!("{}", sum);
}

fn remove_non_number(nums: &Vec<&str>) -> Vec<i32> {
	let mut res: Vec<i32> = Vec::new();
	for x in nums {
		if x.trim() != "0" && x.trim() != "" {
			let num: i32 = x.parse::<i32>().unwrap();
			res.push(num);
		}
	}
	res
}