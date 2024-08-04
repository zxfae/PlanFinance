use regex::Regex;

//Rust.doc (-) currentFormat
pub fn date_test(date: &str) -> bool {
    let date_regex = Regex::new(r"^\d{2}-\d{2}-\d{4}$").unwrap();
    date_regex.is_match(date)
}

pub fn auto_distribute(total: i32) -> (i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) {
    let mut distribution = (total / 12) as i8;
    let mut modulo = (total % 12) as i8;

    let mut months = [distribution; 12];

    for i in 0.. modulo {
        months[i as usize] += 1;
    }

    (
        months[0], months[1], months[2], months[3], months[4],
        months[5], months[6], months[7], months[8], months[9],
        months[10], months[11]
    )
}