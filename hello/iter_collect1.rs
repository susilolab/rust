// `collect` mengkonsumsi iterator dan mengembalikan data berupa array/vector
fn main() {
    let data = [1, 2, 3, 4, 5];
    let data_string = ['a', 'b', 'c', 'd'];
    
    simple_collect(&data);
    turbofish_collect(&data);
    make_string_collect(&data_string);
}

// simple `collect` dengan mengalikan 2 pada setiap angka
// dengan fungsi `map`
fn simple_collect(data: &[i32]) {
    let res: Vec<i32> = data.iter().map(|x| x * 2).collect();
    println!("simple_collect: {:?}", res);
}

// hasilnya hampir sama dengan fungsi `simple_collect`
// hanya saja hasil tipe kembalian ditaruh pada `collect`
// atau biasa disebut `turbofish` 
fn turbofish_collect(data: &[i32]) {
    let res = data.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("turbofish_collect: {:?}", res);
}

fn make_string_collect(data: &[char]) {
    let res: String = data
        .iter()
        .map(|&x| x as u8)
        .map(|x| x as char)
        .collect();
    println!("make_string_collect: {}", res);
}