#[derive(Debug)]
struct Char {
    letter: char,
    count: i32,
}

fn main() {
    let s = "AABCCCDEEE";
    let c = s.chars().collect::<Vec<_>>();

    let mut i = 0;
    let mut j = c[0];
    let mut x = 0;
    let mut map: Vec<Char> = Vec::new();
    let cnt = c.len() - 1;
    loop {
        if c[i] == j {
            x += 1;
        } else {
            map.push(Char {
                letter: j,
                count: x,
            });
            j = c[i];
            x = 1;
        }

        if i == cnt {
            map.push(Char {
                letter: j,
                count: x,
            });
            break;
        }

        i += 1;
    }

    println!("source: {}", s);
    println!("result: \n{:?}", map);
}
