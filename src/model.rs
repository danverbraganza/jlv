#[derive(Debug)]
pub struct Record {
    pub seq_no: usize,
    pub raw: String,
}

impl Record {
    // Generate a single record from input line
    pub fn from_str(i: usize, str: &str) -> Record {
        Record {
            seq_no: i,
            raw: str.to_string(),
        }
    }
}
