use time_warp::datetime;

fn main() {
    let new_date = datetime::DateTime::now();
    println!("The new date is {:?}", new_date);
    println!("the main function has run");
}