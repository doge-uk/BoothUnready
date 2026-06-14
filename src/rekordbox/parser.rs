use std::path::{Path, PathBuf};

/// Extract all file paths from a rekordbox XML file.
pub fn extract_file_paths(xml_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let xml_str = std::fs::read_to_string(xml_path)?;
    extract_file_paths_from_str(&xml_str)
}

pub fn extract_file_paths_from_str(xml_str: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut paths = Vec::new();
    let bytes = xml_str.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        if i + 10 < len && &bytes[i..i + 10] == b"Location=\"" {
            i += 10;
            let start = i;
            while i < len && bytes[i] != b'"' {
                i += 1;
            }
            let raw_location = &xml_str[start..i];

            let path_str = raw_location
                .strip_prefix("file://localhost/")
                .or_else(|| raw_location.strip_prefix("file:///"))
                .unwrap_or(raw_location);

            let decoded = url_decode(path_str);
            paths.push(PathBuf::from(decoded));
        } else {
            i += 1;
        }
    }

    Ok(paths)
}

fn url_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = u8::from_str_radix(&input[i + 1..i + 3], 16) {
                result.push(hex as char);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }

    result
}