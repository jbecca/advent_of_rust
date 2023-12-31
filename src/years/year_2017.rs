use std::error::Error;

pub mod day1;

pub(crate) fn run_day(day: &u16) -> Result<(), Box<dyn Error>> {
    match day {
        1 => Ok(day1::run_both_parts()?),
        _ => Ok(println!("Invalid day")),
    }
}
