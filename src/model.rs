#[derive(Debug)]
pub struct Record {
    pub raw: String,
}

impl Record {
    // Generate a single record from input line
    pub fn from_str(str: &str) -> Record {
        Record {
            raw: str.to_string(),
        }
    }
}
