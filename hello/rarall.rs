use std::fs;
use std::path::Path;

fn is_exe(fname: &str) -> bool {
	let name = Path::new(fname);

	name.extension().unwrap().to_str().unwrap() == "exe"
}

fn main() -> std::io::Result<()> {
	let dirname = r#"D:\var\Downloads\Programs"#;
	// for entry in fs::read_dir(dirname)? {
	// 	let dir = entry?;
	// 	println!("{}", dir.path().display().to_string());
	// }
	let entries = fs::read_dir(dirname)?
		.filter_map(Result::ok)
		.fold(Vec::new(), |mut acc, x| {
			acc.push(x.path().display().to_string());
			acc
		})
		.iter()
		.filter(|x| {
			let path = Path::new(x);
			path.is_file() == true
		})
		.filter(|x| is_exe(x) == true)
		.for_each(|e| println!("{}", e));
		// .map(|res| {
		// 	println!("res: {:?}", res);
		// 	res.map(|e| {
		// 		println!("e: {:?}", e);
		// 		e.path()
		// 	})
		// 	.iter().filter(|e| e.extension().unwrap() == "exe")
		// })
		// .filter(|res| res)
		// .collect::<Result<Vec<_>, std::io::Error>>()?;

	println!("{:?}", entries);

	Ok(())
}
