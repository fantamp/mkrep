pub struct Record {
    pub year: u32,
    pub week: u8,
    pub title: String,
    pub main_text: String,
    pub addit_text: Option<String>,
    pub rank: u32,
}

pub type Records = Vec<Record>;

pub struct Topic {
    pub title: String,
    pub rank: u32,
    pub records: Records,
}

pub struct Report {
    pub rcpt: String,
    pub topics: Vec<Topic>,
}

#[cfg(test)]
pub fn create_dummy_record(title: &str, main_text: &str) -> Record {
    Record {
        year: 2018,
        week: 52,
        title: title.to_string(),
        main_text: main_text.to_string(),
        addit_text: Some(
            "Хрючит черный поросенок на бежевой подстилке"
                .to_string(),
        ),
        rank: 1,
    }
}

#[cfg(test)]
pub fn create_dummy_topic() -> Topic {
    let r1 = create_dummy_record(
        "Солнце",
        "Вращается вокруг Земли в периодом 1 год",
    );
    let r2 = create_dummy_record("Луна", "Является естественным спутником Земли и всего-то повернута одной стороной");
    let recs = vec![r1, r2];
    Topic {
        title: "Дела космические".to_string(),
        rank: 1,
        records: recs,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn create_record() {
        super::create_dummy_record("Иван", "Станция Боровское Шоссе, уважаемые пассажиры, при выходе их подеза не забываейте свои вещсчи. Острожно, двери закрываются, следующая станция");
    }
}
