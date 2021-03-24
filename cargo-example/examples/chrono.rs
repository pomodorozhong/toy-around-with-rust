use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    println!("it's {}.", utc);
}
