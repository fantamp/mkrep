use clap::{Arg, App};

pub struct Args {
    pub path: String,
    pub worksheet: String,
    pub year_week: Option<(u32, u8)>,
    pub rcpt: String,
}

pub fn get_args() -> Result<Args, String> {
    let check_year = |x: String| -> Result<(), String> {
        x.parse::<u32>().map(|_|()).map_err(|x| x.to_string())
    };

    let check_week = |x: String| -> Result<(), String> {
        let w = x.parse::<u8>().map_err(|x| x.to_string())?;
        if w < 1 || w > 52 {
            Err("Week number must be between 1 and 52".to_string())
        } else {
            Ok(())
        }
    };

    let matches = App::new("Make beautiful text report from Excel files")
        .about("Does awesome things")
        .arg(Arg::with_name("file").required(true).help("Path to Excel file to read data from"))
        .arg(Arg::with_name("worksheet").required(true))
        .arg(Arg::with_name("rcpt").required(true))
        .arg(Arg::with_name("year").long("year").validator(check_year).requires("week").takes_value(true))
        .arg(Arg::with_name("week").long("week").validator(check_week).requires("year").takes_value(true))
        .get_matches();

    let get = |a: &str| {
        matches.value_of(a).ok_or_else(|| format!("failed to read arg {}", a))
    };

    let year_week = match (matches.value_of("year"), matches.value_of("week")) {
        (Some(y_str), Some(w_str)) => {
            let year = y_str.parse::<u32>().expect("failed to parse year");
            let week = w_str.parse::<u8>().expect("failed to parse week");
            Some((year, week))
        },
        _ => None
    };

    let a = Args {
        path: get("file")?.to_string(),
        worksheet: get("worksheet")?.to_string(),
        year_week,
        rcpt: get("rcpt")?.to_string()
    };

    Ok(a)
}
