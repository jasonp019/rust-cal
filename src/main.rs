use ansi_term::Colour::{Blue, Red, Yellow};
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut test_date: CalDate;

    test_date = CalDate {
        year: 2023,
        month: 09,
        day: 01,
    };

    println!("Checking date: {}/{}/{}", test_date.month, test_date.day, test_date.year);
    println!("  - day of week: {}", day_of_week(&test_date));

    println!("{}", MonthFormatter::month_str(&test_date));

    test_date = CalDate {
        year: 1900,
        month: 02,
        day: 27,
    };

    println!("Checking date: {}/{}/{}", test_date.month, test_date.day, test_date.year);
    println!("  - day of week: {}", day_of_week(&test_date));

    println!("{}", MonthFormatter::month_str(&test_date));

    test_date = CalDate {
        year: 2024,
        month: 06,
        day: 18,
    };

    println!("Checking date: {}/{}/{}", test_date.month, test_date.day, test_date.year);
    println!("  - day of week: {}", day_of_week(&test_date));

    println!("{}", MonthFormatter::month_str(&test_date));
}

// convert y, m, d into integer day of week (1-based)
fn day_of_week(i_date: &CalDate) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = i_date.year;
    
    println!("in day_of_week; date={}/{}/{}", i_date.month, i_date.day, y);

    if (i_date.month < 3) {
        y -= 1;
    }

    let dow: usize = ((y + (y / 4) - (y / 100) + (y / 400) + t[(i_date.month - 1) as usize] + i_date.day as u16) % 7) as usize;

    (dow + 1)
}

fn fmt_cal(i_month: &u16) -> String {
    let mut cal_str = String::from("");

    return cal_str;
}

struct CalDate {
    year: u16,
    month: u8,
    day: u8,
}

struct MonthFormatter {}

lazy_static! {
    static ref MONTH_NAMES: Vec<&'static str> = [
        "",
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ]
    .to_vec();
}

lazy_static! {
    static ref MONTH_NAMES_SHORT: Vec<&'static str> =
        ["", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
            .to_vec();
}

lazy_static! {
    static ref DOW_NAMES_SHORT: Vec<&'static str> =
        ["", "Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].to_vec();
}        

impl MonthFormatter {
    pub fn month_str(in_date: &CalDate) -> String {
        let mut fmt_mo = String::from("");
        let cal_date_start = CalDate {
            year: in_date.year,
            month: in_date.month,
            day: 1,
        };
        let start_dow = day_of_week(&cal_date_start);
        let mut dow = start_dow;
        let last_days = HashMap::from([
            (1, 31), (3, 31), (5, 31), (7, 31), (8, 31), (10, 31), (12, 31),
            (4, 30), (6, 30), (9, 30), (11, 30),
        ]);
        let last_day_of_month: usize;

        if (last_days.contains_key(&cal_date_start.month)) {
            last_day_of_month = *(last_days.get(&cal_date_start.month).unwrap()) as usize;
        }
        else {
            if ((cal_date_start.year % 4 > 0) ||
                ((cal_date_start.year % 100 == 0) &&
                 (cal_date_start.year % 400 > 0))) {
                last_day_of_month = 28;
            }
            else { last_day_of_month = 29; }
        }
        
        fmt_mo = std::format!(
            "{:^21} ({})",
            (MONTH_NAMES[in_date.month as usize].to_owned() + " " + in_date.year.to_string().as_str()), dow
        );
        fmt_mo += "\n";

        // print the names of the days of the week
        // why the f*** did Rust adopt python's < upper bound sh1te?
        for n in 1..8 {
            fmt_mo += std::format!("{:<3}", DOW_NAMES_SHORT[n].to_string()).as_str();
        }
        fmt_mo += "\n";

        // skip to the first day of week
        for _n in 1..start_dow {
            fmt_mo += "   ";
        }
        
        // append the days to the month string
        for n in 1..(last_day_of_month+1) {
            fmt_mo += std::format!("{:>2} ", n.to_string()).as_str();

            if (dow % 7 == 0) {
                fmt_mo += "\n";
            }

            dow += 1;
        }
        
        return fmt_mo;
    }
}
