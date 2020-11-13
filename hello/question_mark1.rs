use std::fs::File;

struct Hello(Option<String>);

fn main() -> std::io::Result<()> {
	let a = Hello(None);
    let f = a?;

    Ok(())
}
