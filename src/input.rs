// The input module deals with loading a file/stdin/whatever, and producing Records

use crate::model::Record;

pub fn records_from_file(filename: &str) -> Vec<Record> {
    // TODO(danver): Make this not spool
    let content = std::fs::read_to_string(filename).expect("Failed to read the file");

    let mut output = vec![];

    for line in content.lines() {
        output.push(Record::from_str(line));
    }

    output
}
