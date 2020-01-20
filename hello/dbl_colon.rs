// pada rust 2015 kode sukses dikompile
// rustc --edition=2015 --out-dir=bin dbl_colon.rs
//
// pada rust 2018 error karena tidak ditemukan crate root `a`.
//
mod a {
    pub fn foo() {
        println!("a::foo() dipanggil.");
    }
}

// edisi 2015 masih menunjuk ke root crate
// mod b {
//     pub fn foo() {
//         ::a::foo();
//     }
// }

// edisi 2018 sudah diganti dengan `crate`
mod b {
	pub fn foo() {
		crate::a::foo();
	}
}

fn main() {
    b::foo();
}
