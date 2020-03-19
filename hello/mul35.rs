/// Jika kita mencantumkan semua bilangan asli di bawah 10 yang merupakan kelipatan 3 atau 5, 
/// kita mendapatkan 3, 5, 6 dan 9. Jumlah kelipatan ini adalah 23.
/// Temukan jumlah semua kelipatan 3 atau 5 di bawah 1000.
/// bil yg merupakan kelipatan 3 adalah yg habis dibagi 3
fn main() {
    let mut res = 0;
    for i in 1..1000 {
    	if i % 3 == 0 || i % 5 == 0 {
    		res += i;
    	} else {
    		continue;
    	}
    }
    println!("{:?}", res);
}
