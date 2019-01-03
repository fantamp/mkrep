use crate::data::*;
use std::cmp::min;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

type RawRecord = HashMap<String, String>;
type RawFields = Vec<String>;

pub fn load_report_from_table<'a, I>(
    raw_fileds: I,
    year: u32,
    week: u8,
    rcpt: &str,
) -> Result<Report, String>
where
    I: Iterator<Item = &'a RawFields>,
{
    let mut topics: HashMap<String, Topic> = HashMap::new();

    for rr in FieldsToRawRecord::new(raw_fileds) {
        let rec = raw_record_to_record(&rr, rcpt)?;

        if rec.rank > 0 && rec.year == year && rec.week == week {
            let topic_name = rr
                .get("Topic")
                .ok_or_else(|| "records must have 'Topic' field".to_string())?;
            // FIXME: why .clone() ?
            let topic = topics
                .entry(topic_name.clone())
                .or_insert_with(|| make_topic(&topic_name));
            topic.records.push(rec);
        }
    }

    let mut tv: Vec<Topic> = topics.drain().map(|x| x.1).collect();

    // sort records in topics by their rank
    for t in &mut tv {
        t.records.sort_by_key(|x| x.rank);
        t.records.reverse();
    }

    // sort topics by rank of the highest record
    tv.sort_by_key(|x| x.records[0].rank);
    tv.reverse();

    let rep = Report {
        rcpt: rcpt.to_string(),
        topics: tv,
    };

    Ok(rep)
}

fn get(rr: &RawRecord, field_name: &str) -> Result<String, String> {
    Ok(rr
        .get(field_name)
        .ok_or_else(|| format!("records must have '{}' field", field_name))?
        .clone())
}

fn get_year_and_week(rr: &RawRecord) -> Result<(u32, u8), String> {
    let year: u32 = get(rr, "Year")?
        .parse()
        .map_err(|e| format!("failed to parse Year field value: {}", e))?;
    let week: u8 = get(rr, "Week")?
        .parse()
        .map_err(|e| format!("failed to parse Week field value: {}", e))?;
    Ok((year, week))
}

fn raw_record_to_record(rr: &RawRecord, rcpt: &str) -> Result<Record, String> {
    fn make_rank(s: &str) -> Result<u32, String> {
        match s {
            "" => Ok(0),
            "x" => Ok(1),
            _ => Ok(s.parse::<u32>().map_err(|x| {
                format!(
                    "failed to parse '{}' value in Rank field: {}",
                    s,
                    x.to_string()
                )
            })?),
        }
    }

    let (year, week) = get_year_and_week(rr)?;
    let title = get(rr, "Title")?;
    let main_text = get(rr, "Text")?;
    let addit_text = rr.get(&format!("{}:Text", rcpt)).map(|x| x.clone());
    let rank = make_rank(&get(rr, &format!("{}:Rank", rcpt))?)?;

    let rec = Record {
        year,
        week,
        title,
        main_text,
        addit_text,
        rank,
    };

    Ok(rec)
}

struct FieldsToRawRecord<'a, I>
where
    I: Iterator<Item = &'a RawFields>,
{
    rows: I,
    header: Option<RawFields>,
}

impl<'a, I> FieldsToRawRecord<'a, I>
where
    I: Iterator<Item = &'a RawFields>,
{
    fn new(rows: I) -> FieldsToRawRecord<'a, I> {
        FieldsToRawRecord {
            rows: rows,
            header: None,
        }
    }
}

impl<'a, I> Iterator for FieldsToRawRecord<'a, I>
where
    I: Iterator<Item = &'a RawFields>,
{
    type Item = RawRecord;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fields) = self.rows.next() {
            if let Some(h) = &self.header {
                let mut rr = RawRecord::new();
                for i in 0..min(h.len(), fields.len()) {
                    rr.insert(h[i].clone(), fields[i].clone());
                }
                return Some(rr);
            } else {
                self.header = Some(fields.clone());
                return self.next();
            }
        } else {
            return None;
        }
    }
}

pub fn get_recent_year_and_week<'a, I>(rows: I) -> Result<Option<(u32, u8)>, String>
where
    I: Iterator<Item = &'a RawFields>,
{
    let mut pairs: Vec<(u32, u8)> = Vec::new();

    for rr in FieldsToRawRecord::new(rows) {
        pairs.push(get_year_and_week(&rr)?);
    }

    pairs.sort_by_key(|p| p.0 * 100 + p.1 as u32);

    Ok(pairs.pop())
}

#[allow(dead_code)]
fn load_report_from_stream(
    f: &mut Read,
    year: u32,
    week: u8,
    rcpt: &str,
) -> Result<Report, String> {
    let fields: Vec<Vec<String>> = BufReader::new(f)
        .lines()
        .map(|x| split_line(&x.expect("testing expect")))
        .collect();
    load_report_from_table(fields.iter(), year, week, rcpt)
}

fn make_topic(title: &str) -> Topic {
    Topic {
        title: title.to_string(),
        rank: 0,
        records: Vec::new(),
    }
}

/// Make RawFields from tab-separated report line
///
/// # Examples
///
/// let rec = split_line("Year\tWeek\tTopic\tTitle\tText\tSH:Prio\tTeam:prio");
/// assert_eq!(rec[0], "Year");
/// assert_eq!(rec[6], "Team:prio");
/// assert_eq!(rec.len(), 7);
fn split_line(line: &str) -> RawFields {
    line.split('\t').map(|x| x.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::split_line;
    use crate::test_data::get_test_file_text;
    use std::io::Cursor;
    use std::io::Write;

    #[test]
    fn test_load_report_from_table() {
        let mut table = Vec::new();
        table.push(split_line(
            "Year\tWeek\tTopic\tTitle\tText\tSH:Rank\tTeam:Rank\tSH:Text\tTeam:Text",
        ));
        table.push(split_line("2018\t52\tЛюди\tНайм: статус\tКрасота зимнего периода состоит в особом\t1\t1\t\t"));

        let rep =
            super::load_report_from_table(table.iter(), 2018, 52, "SH").expect("loading error");
        assert_eq!(rep.topics.len(), 1);
    }

    #[test]
    fn load_from_stream() {
        let mut buf = Cursor::new(Vec::new());
        assert!(write!(&mut buf, "{}", get_test_file_text()).is_ok());
        buf.set_position(0);
        let rep = super::load_report_from_stream(&mut buf, 2018, 52, "SH").expect("loading error");
        assert_eq!(rep.topics.len(), 3);
        assert_eq!(rep.topics[0].title, "Crust");
        assert_eq!(rep.topics[0].records[0].title, "Snowball earth");
    }

}
