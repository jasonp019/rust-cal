fn main() {
    println!("Hello, world!");
    println!("Checking date: 1/1/2023");
    println!("  - day of week: {}", day_of_week(2023, 1, 1));
}

// convert y, m, d into integer day of week (0-based)
fn day_of_week(year: usize, month: usize, day: usize) -> usize {
    let T = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;

    println!("in day_of_week; date={}/{}/{}", month, day, year);

    if (month < 3) {
        y -= 1;
    }

    (y + (y / 4) - (y / 100) + (y / 400) + T[(month - 1)] + day) % 7
}

/*
dayofweek(y, m, d)	/* 1 <= m <= 12,  y > 1752 (in the U.K.) */
{
    static int t[] = {0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4};
    if ( m < 3 )
    {
        y -= 1;
    }
    return (y + y/4 - y/100 + y/400 + t[m-1] + d) % 7;
}
*/
