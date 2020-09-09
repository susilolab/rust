fn main() {
	assert_eq!(2_203_160, sum_of_multiples(10_000, &[43, 47]));
}

fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
	let mut res = 0;
    for i in 1..=10 {
    	if i % 3 == 0 || i % 5 == 0 {
    		res += i;
    	} else {
    		continue;
    	}
    }
}
