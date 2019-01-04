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
