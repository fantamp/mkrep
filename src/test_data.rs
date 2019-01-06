use crate::data::*;
use std::io::Cursor;
use std::io::Write;

#[cfg(test)]
pub fn get_test_file_text() -> &'static str {
    "Year\tWeek\tTopic\tTitle\tText\tSH:Rank\tTeam:Rank\tSH:Text\tTeam:Text\n\
    2018\t52\tЛюди\tНайм: статус\tКрасота зимнего периода состоит в особом\t1\t1\t\t\n\
    2018\t52\tCrust\tNot so important subtopic\tBlah blah blah\t5\t1\t\t\n\
    2018\t52\tCrust\tSnowball earth\tEarth had gone through long period of glaciers\t10\t1\tThere are doubts in this theory yet\t\n\
    2018\t52\tРазное\tПерове января\tпосле тридцатьпервого декабря пришло первое января\t1\t1\t\t"
}

#[cfg(test)]
pub fn get_test_file_text_with_bad_year() -> &'static str {
    "Year\tWeek\tTopic\tTitle\tText\tSH:Rank\tTeam:Rank\tSH:Text\tTeam:Text\n\
    zzz\t52\tЛюди\tНайм: статус\tКрасота зимнего периода состоит в особом\t1\t1\t\t\n"
}

#[cfg(test)]
pub fn get_test_file_text_with_bad_week() -> &'static str {
    "Year\tWeek\tTopic\tTitle\tText\tSH:Rank\tTeam:Rank\tSH:Text\tTeam:Text\n\
    2019\tпервая неделя\tЛюди\tНайм: статус\tКрасота зимнего периода состоит в особом\t1\t1\t\t\n"
}

#[cfg(test)]
pub fn make_dummy_report() -> Report {
    load_report_from_string(get_test_file_text())
}

#[cfg(test)]
pub fn load_report_from_string(s: &str) -> Report {
    let mut buf = Cursor::new(Vec::new());
    assert!(write!(&mut buf, "{}", s).is_ok());
    buf.set_position(0);
    crate::read::load_report_from_stream(&mut buf, 2018, 52, "SH").expect("loading error")
}
