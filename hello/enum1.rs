#[derive(Debug)]
enum IpAddrKind {
	V4,
	V6,
}

#[derive(Debug)]
struct IpAddr {
	kind: IpAddrKind,
	address: String,
}

fn main() {
	let four = IpAddrKind::V4;
	println!("{:?}", four);

	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};

	println!("{:?}", loopback);
}