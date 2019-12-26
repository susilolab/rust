/// Contoh penggunaan Option pada rust karena tidak ada null/nil
/// Option sebagai parameter yang bisa berisi "nilai" atau "kosong"/None
///
struct Person {
    fname: &'static str,
    lname: &'static str,
    mname: Option<&'static str>,
}

fn main() {
    // Mengambil nilai dari Option dengan "if let"
    // atau bisa juga dengan "unwrap" atau "unwrap_or"
    if let Some(name) = get_name(1) {
        println!("Nama Anda: {}", name);
    }

    if let None = get_name(0) {
        println!("Nama tidak ketemu!.");
    }

    let person1 = Person {
        fname: "Agus",
        lname: "Susilo",
        mname: None,
    };
    println!("{}", get_fullname(person1));

    let person2 = Person {
        fname: "Agus",
        lname: "Susilo",
        mname: Some("Berbah"),
    };
    println!("{}", get_fullname(person2));
}

// Fungsi get_name dapat mengembalikan string atau None
fn get_name(id: i32) -> Option<&'static str> {
    if id > 0 {
        return Some("Agus Susilo");
    }
    None
}

// Fungsi get_fullname dapat memproses dengan nama tengah tidak diisi atau
// dengan nama tengah diisi.
fn get_fullname(p: Person) -> String {
    match p.mname {
        Some(name) => format!("{} {} {}", p.fname, name, p.lname),
        None => format!("{} {}", p.fname, p.lname),
    }
}
