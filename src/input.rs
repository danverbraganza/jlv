// The input module deals with loading a file/stdin/whatever, and producing Records

use std::io::Error;

use crate::model::Record;

pub fn records_from_file(filename: &str) -> Result<Vec<Record>, Error> {
    let content = std::fs::read_to_string(filename)?;

    let mut output = vec![];

    for (i, line) in content.lines().enumerate() {
        output.push(Record::from_str(i, line));
    }

    Ok(output)
}
