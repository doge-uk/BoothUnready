mod batch;
mod config;
mod rekordbox;
mod report;
mod scanner;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the folder to scan
    #[arg(short, long, value_name = "FOLDER_PATH")]
    path: Option<PathBuf>,

    /// Optional: The name of a specific Pioneer DJ device to check compatibility against
    #[arg(short, long, value_name = "DEVICE_NAME")]
    device: Option<String>,

    /// Parse a rekordbox XML file and scan the tracks listed in it
    #[arg(short, long, value_name = "XML_PATH")]
    xml: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let all_devices = config::get_devices();

    let devices_to_scan = if let Some(ref device_name) = cli.device {
        if let Some(device) = all_devices.iter().find(|d| d.name.eq_ignore_ascii_case(device_name)) {
            vec![device.clone()]
        } else {
            eprintln!("Error: Device '{}' not found.", device_name);
            eprintln!("Available devices:");
            for device in all_devices {
                eprintln!("- {}", device.name);
            }
            return;
        }
    } else {
        all_devices
    };

    let results = if let Some(ref xml_path) = cli.xml {
        // Parse XML and scan those specific files
        match rekordbox::extract_file_paths(xml_path) {
            Ok(paths) => {
                let mut all_results = Vec::new();
                for path in &paths {
                    if path.exists() {
                        all_results.push(scanner::scan_file(path, &devices_to_scan));
                    }
                }
                all_results
            }
            Err(e) => {
                eprintln!("Failed to parse XML: {}", e);
                return;
            }
        }
    } else if let Some(ref folder_path) = cli.path {
        batch::batch_scan(folder_path, &devices_to_scan)
    } else {
        eprintln!("Error: Provide either --path <FOLDER> or --xml <XML_FILE>");
        return;
    };

    let report = report::generate_report(&results);
    println!("{}", report);
}