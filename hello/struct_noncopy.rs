#![allow(dead_code)]
/// Buat latihan tapi masih error saat dikompile
///
/// agar bisa dikompile maka pake pattern matching pada
/// parameter function pd kasus ini adalah `xs`
///
/// macro `#![allow(dead_code)]` mengijinkan kode yang tidak digunakan tanpa peringatan
///
struct NonCopy;

// kode lama:
// `fn head(xs: [NonCopy; 1]) -> NonCopy {`
// kode baru dan fixed
fn head([xs]: [NonCopy; 1]) -> NonCopy {
	// `xs[0]` adalah syntactic sugar dari *container.index(index)
	// atau *NonCopy.index(index)
	// kode lama: `xs[0]`
	// kode baru:
	xs
}

fn main() {
}
