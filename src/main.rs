use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(name = "jlv", version, about = "JsonL viewer", long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[arg(value_name = "FILENAME")]
    filename: Option<String>,

    /// Or as flag: `jlv view -f myfile`
    #[arg(short = 'f', long = "filename", value_name = "FILENAME")]
    filename_flag: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        eprintln!("Debug mode on");
    }

    let filename = cli.filename.or(cli.filename_flag);

    match filename {
        Some(fname) => start_view(&fname),
        None => {
            eprintln!("Error: No filename provided.");
            std::process::exit(1);
        }
    }
}

// Initializes the TUI view to view a given filename
fn start_view(filename: &str) {
    println!("Opening filename {filename}");

    // Open the file passed in.
    let content = std::fs::read_to_string(filename).expect("Failed to read the file");

    // Let's iterate through each line and print it with line numbers.
    for (index, line) in content.lines().enumerate() {
        println!("{}: {}", index + 1, line);
    }
}
