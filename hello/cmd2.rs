// use std::process::Command;

fn main() {
//ffmpeg -i "001_Minhajulmuslim Bab Adab Pasal Ke-1 Adab Niat.MP4" -b:a 192K -vn "001_Minhajul-muslim-Bab-Adab Pasal Ke-1 Adab Niat.mp3"

	let files = vec![
		"001_Minhajulmuslim Bab Adab Pasal Ke-1 Adab Niat.MP4",
		"002_Minhajul Muslim Bab Adab Pasal Ke-2 Adab Terhadap Allah SWT.MP4",
		"003_Minhajulmuslim, Bab Adab, Pasal 3- Adab Terhadap Kalamullah (Al-Qur'anul Karim).MP4",
		"004_Minhajulmuslim, Bab Adab, Pasal Ke-4- Adab Terhadap Rasulullah SAW.MKV",
		"005_Minhjulmuslim Bab Adab Pasal Ke-5 Adab Terhadap Diri Sendiri.MP4",
	];
	let path = "C:/Users/susilo/Downloads/Video/Others/Minhajul Muslim";
	for x in files {
		println!("{}", path.to_owned()+x);
	}
	// let mut child = Command::new("ls")
	// 	.arg("D:/")
	// 	.spawn()
	// 	.expect("failed to execute child");

	// let ecode = child.wait().expect("failed to wait on child");
	// assert!(ecode.success());
}