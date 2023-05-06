use time_warp::datetime;

fn main() {
    let first_date = datetime::DateTime {
        year: 1990,
        month: 12,
        day: 0,
        hour: 12,
        minute: 45,
        second: 50,
        weekday: 0,
    };
    let second_date = datetime::DateTime {
        year: 1990,
        month: 12,
        day: 0,
        hour: 12,
        minute: 45,
        second: 45,
        weekday: 0,
    };
    let result = first_date.add(second_date);
    println!("{:?}", result);
}