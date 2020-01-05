/// for dan iterator
///
/// for in dapat digunakan untuk berinteraksi dengan `Iterator`
/// dalam beberapa cara.
/// jika tidak disebutkan(not specified) maka loop akan menerapkan fungsi `into_iter`
/// pada koleksi untuk mengubah kedalam iterator
///
/// Ada 3 fungsi yang akan mengembalikan pandangan yg berbeda dari data pada koleksi
/// atau collection.
///
fn main() {
	println!("for name in names.iter()");
	for_iter();
	println!("\n");

	println!("for name in names.into_iter()");
	for_into_iter();
	println!("\n");

	println!("for name in names.iter_mut()");
	for_iter_mut();
}

// 1. iter
// `iter` akan meminjam setiap elemen dari kumpulan melalui setiap pengulangan.
// dengan demikian membiarkan kumpulan tidak tersentuh dan tersedia untuk digunakan
// kembali setelah pengulangan.
fn for_iter() {
	let names = vec!["Agus", "Susilo", "Nurhayati"];

	for name in names.iter() {
		match name {
			&"Nurhayati" => println!("Halo sayangku!"),
			_ => println!("Hello {}", name),
		}
	}
}

// 2. into_iter
// `into_iter` memakai koleksi data pada setiap perulangan pada data yg disediakan.
// setelah data selesai dipakai(consume) data tsb tidak lagi tersedia untuk digunakan
// kembali karena sudah dipindahkan kedalam perulangan
fn for_into_iter() {
	let names = vec!["Agus", "Susilo", "Nurhayati"];

	for name in names.into_iter() {
		match name {
			// bandingkan dengan kode diatas, tidak ada tanda & (borrow)
			"Nurhayati" => println!("Halo sayangku!"),
			_ => println!("Hello {}", name),
		}
	}
}

// 3. iter_mut
// `iter_mut` meminjam data dan mungkin merubah isinya pada setiap elemen pada koleksi
// mengijinkan koleksi untuk dirubah pada tempatnya.
fn for_iter_mut() {
	let mut names = vec!["Agus", "Susilo", "Nurhayati"];

	for name in names.iter_mut() {
		*name = match name {
			// bandingkan dengan kode diatas, tidak ada tanda & (borrow)
			&mut "Nurhayati" => "Halo sayangku!",
			_ => "Hello",
		}
	}
	println!("names: {:?}", names);
}