use chrono::prelude::*;
use chrono::offset::LocalResult;

pub fn sekarang() {
    let utc: DateTime<Utc> = Utc::now();
    println!("{:?}", utc);

    let dt: DateTime<Local> = Local::now();
    println!("{:?}", dt);

    let dt1 = Utc.ymd(10, 1, 1).and_hms(10, 10, 5);
    println!("{:?}", dt1);
}
