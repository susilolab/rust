use std::io;

fn cek_prima(bil: i32) -> i32 {
	let mut bagi: i32 = 3;
	let mut batas: i32 = 0;

	let prima = if bil == 1 {
		0
	}else if bil == 2 || bil == 3 {
		1
	}else if bil % 2 ==0 {
		0
	}else {
		while batas > bagi {
			if bil % bagi == 0 {
				return 0
			}
			batas = bil/bagi;
			bagi += 2;
		}
		return 1;
	};
	prima
}

fn main() {
	let mut prima = 0;
	let mut rentang = String::new();
	println!("Masukan rentang");
	io::stdin().read_line(&mut rentang)
		.expect("failed to read line");
	let cnt = rentang.trim().parse::<i32>().unwrap();

	for i in 0..cnt {
		prima = cek_prima(i);
		if prima == 1 {
			println!("{}", i);
		}
	}
}
