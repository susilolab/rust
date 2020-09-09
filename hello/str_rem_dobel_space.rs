fn main() {
	let file_name = "001_Minhajulmuslim  Bab Adab Pasal  Ke-1 Adab Niat.MP4";
	println!("{}", file_name.to_lowercase().replace("  ", "_"));
	let a = file_name.split(" ").collect::<String>();
	println!("{:?}", a);
}