use std::io;
use std::io::Write;

struct Baby {
    name: String,
    past_weight: i32,
    present_weight: i32,
}

fn main() {
    // let mut data: Vec<Baby> = Vec::new();
    // br(None);
    // let ask_cnt = prompt("Masukan jumlah data yang akan di input: ");
    // let cnt = ask_cnt.parse::<i32>().unwrap();

    // for _x in 0..cnt {
    //     let name = prompt("Masukan nama bayi: ");

    //     let old_weight = prompt("Masukan berat lama: ");

    //     let new_weight = prompt("Masukan berat sekarang: ");

    //     let baby = Baby {
    //         name: name,
    //         past_weight: old_weight.parse::<i32>().unwrap(),
    //         present_weight: new_weight.parse::<i32>().unwrap(),
    //     };
    //     data.push(baby);
    //     br(None);
    // }

    // println!("Name\tBerat1\tBerat2");
    // for b in data {
    //     println!("{}\t{}\t{}", b.name, b.past_weight, b.present_weight);
    // }
}

fn prompt<T>(title: &str) -> T {
    print!("{}", title);
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    name.parse::<T>().unwrap()
}

fn br(col: Option<i32>) {
    let col_num = match col {
        Some(val) => val,
        None => 80,
    };

    for _i in 0..col_num {
        print!("-");
    }
    print!("\n");
}
