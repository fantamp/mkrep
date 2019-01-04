use crate::data::*;
use std::io::Write;

pub fn render(rep: &Report, to: &mut Write) -> Result<(), String> {
    render_html(rep, to).map_err(|e| e.to_string())
}

fn render_html(rep: &Report, to: &mut Write) -> std::io::Result<()> {
    write!(
        to,
        r#"<html>
<head>
<meta charset="utf-8">
<style type="text/css">
  body {{
    font-family: Arial, sans;
  }}
</style>
</head><body>
"#
    )?;

    for t in &rep.topics {
        render_topic(t, to)?;
    }

    write!(to, "</body>\n")?;
    Ok(())
}

fn render_topic(topic: &Topic, to: &mut Write) -> std::io::Result<()> {
    write!(
        to,
        "<h2>{}</h2>\n",
        htmlescape::encode_minimal(&topic.title)
    )?;

    for r in &topic.records {
        render_record(r, to)?
    }
    Ok(())
}

fn render_record(r: &Record, to: &mut Write) -> std::io::Result<()> {
    write!(to, "<h3>{}</h3>\n", htmlescape::encode_minimal(&r.title))?;
    write!(to, "<p>{}</p>\n", htmlescape::encode_minimal(&r.main_text))?;

    if let Some(ref t) = r.addit_text {
        write!(to, "<p>{}</p>\n", htmlescape::encode_minimal(t))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn test_render() {
        let mut rep = crate::test_data::make_dummy_report();

        rep.topics[0].records.push(crate::data::Record {
            year: 2019,
            week: 1,
            title: "Title with <b>html-formatted test</b>".to_string(),
            main_text: "Body".to_string(),
            addit_text: None,
            rank: 1,
        });

        let mut buf = Cursor::new(Vec::new());
        assert!(super::render(&rep, &mut buf).is_ok());
        let s = String::from_utf8_lossy(buf.get_ref());
        assert!(s.find("&lt;b&gt;html").is_some());
        assert!(s.find("utf-8").is_some());
        assert!(s.find("text/css").is_some());
    }
}
