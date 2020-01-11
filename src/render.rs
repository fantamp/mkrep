mod html;
mod text;
use crate::data::*;
use std::io::Write;

pub enum OutputFormat {
    Text,
    Html,
}

pub fn render(rep: &Report, format: OutputFormat, to: &mut dyn Write) -> Result<(), String> {
    match format {
        OutputFormat::Text => text::render(rep, to),
        OutputFormat::Html => html::render(rep, to),
    }
}
