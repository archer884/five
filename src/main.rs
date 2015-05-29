extern crate chrono;
use chrono::{ Duration, NaiveTime, UTC };

fn main() {
    let time = std::env::args().nth(1)
        .and_then(|time| time_from_hours_minutes(&time))
        .unwrap_or(UTC::now().time());

    println!("{}", is_five_somewhere(&time));
}

fn is_five_somewhere(time: &NaiveTime) -> bool {
    let max_difference = Duration::minutes(5);
    let target_time = NaiveTime::from_hms(17, 0, 0);
    let offsets = [
        Duration::minutes(-(12 * 60)),
        Duration::minutes(-(11 * 60)),
        Duration::minutes(-(10 * 60)),
        Duration::minutes(-(9 * 60 + 30)),
        Duration::minutes(-(9 * 60)),
        Duration::minutes(-(8 * 60)),
        Duration::minutes(-(7 * 60)),
        Duration::minutes(-(6 * 60)),
        Duration::minutes(-(5 * 60)),
        Duration::minutes(-(4 * 60 + 30)),
        Duration::minutes(-(4 * 60)),
        Duration::minutes(-(3 * 60 + 30)),
        Duration::minutes(-(3 * 60)),
        Duration::minutes(-(2 * 60)),
        Duration::minutes(-(1 * 60)),
        Duration::minutes(0),
        Duration::minutes(1 * 60),
        Duration::minutes(2 * 60),
        Duration::minutes(3 * 60),
        Duration::minutes(3 * 60 + 30),
        Duration::minutes(4 * 60),
        Duration::minutes(4 * 60 + 30),
        Duration::minutes(5 * 60),
        Duration::minutes(5 * 60 + 30),
        Duration::minutes(5 * 60 + 45),
        Duration::minutes(6 * 60),
        Duration::minutes(6 * 60 + 30),
        Duration::minutes(7 * 60),
        Duration::minutes(8 * 60),
        Duration::minutes(8 * 60 + 45),
        Duration::minutes(9 * 60),
        Duration::minutes(9 * 60 + 30),
        Duration::minutes(10 * 60),
        Duration::minutes(10 * 60 + 30),
        Duration::minutes(11 * 60),
        Duration::minutes(11 * 60 + 30),
        Duration::minutes(12 * 60),
        Duration::minutes(12 * 60 + 45),
        Duration::minutes(13 * 60),
        Duration::minutes(14 * 60),
    ];

    offsets.iter().any(|&offset| max_difference >= abs_duration(&((*time + offset) - target_time)))
}

fn abs_duration(d: &Duration) -> Duration {
    if *d < Duration::minutes(0) {
        -*d
    } else {
        *d
    }
}

fn time_from_hours_minutes(s: &str) -> Option<NaiveTime> {
    match s.parse() {
        Ok(time) => Some(time),
        Err(_) => {
            let mut s = s.to_string();
            s.push_str(":00");
            s.parse().ok()
        }
    }
}
