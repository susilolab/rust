use std::process::Command;

fn main() {
    let name: &str = "systemctl";
    let args = vec!["list-unit-files", "--type=service", "--state=enabled"];
    run_cmd(name, &args);
}

fn run_cmd(name: &str, args: &Vec<&'static str>) {
    let output = Command::new(name)
        .args(args)
        .output()
        .expect("Tidak dapat menjalankan command.");

    let ls = output.stdout;
    println!("{:#?}", ls.iter().map(|&x| x as char).collect::<String>());
}
