use time_warp;

#[test]
fn creation_works() {
    let new_date = time_warp::datetime::DateTime::now();
    println!("{:?}", new_date);
}