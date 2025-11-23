use crate::input::records_from_file;

// Initializes the TUI view to view a given filename
pub fn start_view(filename: &str) {
    println!("Opening filename {filename}");

    // Open the file passed in.
    let records = records_from_file(filename);

    for (i, record) in records.iter().enumerate() {
        println!("{}: {:#?}", i, record)
    }
}
