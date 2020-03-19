fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    let sum = add(a, b);
    println!("Sum v1: {}", sum);

    let a: u8 = 4;
    let b: u16 = 6;
    let sum = add_v2(a, b);
    println!("Sum v2: {}", sum);

    let a: u8 = 4;
    let b: u16 = 5;
    let sum = add_v3(a, b);
    println!("Sum v3: {}", sum);

    let a: u8 = 1;
    let b: u16 = 2;
    let sum = add_v4(a, b);
    println!("Sum v4: {}", sum);
}

/// Generic dengan satu tipe data T, argumen harus bertipe sama, ex: i32 semua, u16 semua
fn add<T: Into<f64>>(a: T, b: T) -> f64 {
    a.into() + b.into()
}

fn add_v2<A: Into<f64>, B: Into<f64>>(a: A, b: B) -> f64 {
    a.into() + b.into()
}

fn add_v3<A, B>(a: A, b: B) -> f64
where
    A: Into<f64>,
    B: Into<f64>
{
    a.into() + b.into()
}

/// Pake trait From dari pada Into
fn add_v4<A, B>(a: A, b: B) -> f64
where
    f64: From<A> + From<B>
{
    f64::from(a) + f64::from(b)
}