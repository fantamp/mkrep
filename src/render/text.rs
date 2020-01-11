use crate::data::*;
use std::io::Write;

pub fn render_record(r: &Record, to: &mut dyn Write) -> std::io::Result<()> {
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

pub fn render_topic(topic: &Topic, to: &mut dyn Write) -> std::io::Result<()> {
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

pub fn render(rep: &Report, to: &mut dyn Write) -> Result<(), String> {
    for t in &rep.topics {
        render_topic(t, to).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn test_render() {
        let rep = crate::test_data::make_dummy_report();
        let mut buf = Cursor::new(vec![0; 0]);
        assert!(super::render(&rep, &mut buf).is_ok());
    }
}
