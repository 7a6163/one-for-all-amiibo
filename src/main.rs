use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const OUTPUT_FILENAME: &str = "combined.bin";
const MAX_FILE_SIZE: usize = 572;

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().skip(1).collect();
    let input_paths: Vec<PathBuf> = args.iter().map(|s| PathBuf::from(s)).collect();

    // Check that input files exist
    for path in &input_paths {
        if !path.exists() {
            println!("Error: file not found: {:?}", path);
            return Ok(());
        }
    }

    // Open output file for writing
    let output_path = PathBuf::from(OUTPUT_FILENAME);
    let mut output_file = fs::File::create(&output_path)?;

    // Combine files
    for path in &input_paths {
        let mut input_file = fs::File::open(&path)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;

        // Pad data with zeros if necessary
        let output_size = input_data.len().max(MAX_FILE_SIZE);
        let mut output_data = vec![0u8; output_size];
        output_data[..input_data.len()].copy_from_slice(&input_data);

        // Write padded data to output file
        output_file.write_all(&output_data)?;
    }

    println!("Output file written to {:?}", output_path);

    Ok(())
}