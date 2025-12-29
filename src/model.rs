use serde_json::Value;

#[derive(Debug)]
pub struct Record {
    pub seq_no: usize,
    pub raw: String,
    pub value: Option<Value>,
}

impl Record {
    // Generate a single record from input line
    pub fn from_str(i: usize, str: &str) -> Record {
        Record {
            seq_no: i,
            raw: str.to_string(),
            // Derserialize the JSON value
            value: serde_json::from_str(str).ok(),
        }
    }
}

pub trait RecordSource {
    // Returns all the records is this RecordProvider. In the future we might allow you to slice and paginate.
    fn records(&self) -> &[Record];

    fn title(&self) -> String;

    fn iter(&self) -> std::slice::Iter<'_, Record> {
        self.records().iter()
    }
}

struct FileRecordSource {
    title: Box<str>,
    records: Vec<Record>,
}

impl RecordSource for FileRecordSource {
    fn records(&self) -> &[Record] {
        self.records.as_slice()
    }

    fn title(&self) -> String {
        self.title.to_string()
    }
}
