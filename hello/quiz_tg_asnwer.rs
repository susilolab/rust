fn foo<'a, T>(m: &'a [T]) -> Vec<(usize, usize)>
	where &'a T: IntoIterator<Item=&'a u8>
{
	let mut c = Vec::new();
	for (y, n) in m.into_iter().enumerate() {
		for (x, &o) in n.into_iter().enumerate() {
			if o == 1 {
				c.push((x, y));
			}
		}
	}
	c
}

fn main() {

}

#[test]
fn generic_over_any_m_x_n_matrix() {
	foo(&[[1, 0, 0], [1, 1, 1], [0, 0, 0]]);
	foo(&[[1, 1], [1, 1]]);
}