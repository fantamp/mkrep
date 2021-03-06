mod args;
mod data;
mod read;
mod read_excel;
mod render;

#[cfg(test)]
mod test_data;

/*
TODO:
    Backlog
        - support images

    Tech things:
        extract constants to const
        find .clone() and other places where changing to & would be possible

    Learning things:
        √ Why doesn't documentation tests run?
        √ https://doc.rust-lang.org/edition-guide/rust-2018/control-flow/loops-can-break-with-a-value.html
        √ diff between 'extern crate X' and 'use X' and whether the first is mandatory
        √ How to make my own iterator from yielding closure
        √ Organize rendereres in subdirs
        √ Make integration tests
*/

fn main() {
    match do_everything() {
        Err(s) => {
            eprintln!("Error: {}", s);
            std::process::exit(1);
        }
        _ => {}
    }
}

fn do_everything() -> Result<(), String> {
    let a = crate::args::get_args()?;

    let data = read_excel::read_excel_to_vec(&a.path, &a.worksheet)?;

    let yw = if let Some(yw) = a.year_week {
        yw
    } else {
        read::get_recent_year_and_week(data.iter())?
            .ok_or_else(|| "failed to find recent year and week in excel file")?
    };

    let rep = read::load_report_from_table(data.iter(), yw.0, yw.1, &a.rcpt)?;

    let mut stdout = std::io::stdout();
    render::render(&rep, a.format, &mut stdout)?;

    Ok(())
}
