use crate::data::*;
use std::io::Write;

pub fn render_record(r: &Record, to: &mut Write) -> std::io::Result<()> {
    write!(to, "— {}\n", r.title)?;
    write!(to, "{}\n", r.main_text)?;

    match r.addit_text {
        Some(ref t) => {
            write!(to, "{}\n", t)?;
        }
        _ => (),
    }

    write!(to, "\n")?;

    Ok(())
}

pub fn render_topic(topic: &Topic, to: &mut Write) -> std::io::Result<()> {
    write!(to, "{}\n", topic.title)?;
    for _ in 0..topic.title.chars().count() {
        write!(to, "~")?;
    }
    write!(to, "\n\n")?;

    for r in &topic.records {
        render_record(r, to)?
    }
    write!(to, "\n")?;
    Ok(())
}

pub fn render_report(rep: &Report, to: &mut Write) -> Result<(), String> {
    for t in &rep.topics {
        render_topic(t, to).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn test_render_record() {
        super::create_dummy_record("Иван", "Станция Боровское Шоссе, уважаемые пассажиры, при выходе их подеза не забываейте свои вещсчи. Острожно, двери закрываются, следующая станция");
    }

    #[test]
    fn render_topic() {
        let mut buff = Cursor::new(vec![0; 0]);
        let t = super::create_dummy_topic();
        assert!(super::render_topic(&t, &mut buff).is_ok());
    }

}
