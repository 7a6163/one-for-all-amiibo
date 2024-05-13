use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // Check that we have at least two arguments for input files
    if args.len() < 2 {
        eprintln!("Usage: one-for-all-amiibo <input1> <input2> [<output>]");
        return Ok(());
    }

    // Determine the output file name
    let default_output = "combined.bin".to_string();
    let output_file_name = args.last().unwrap_or(&default_output);

    // Collect input paths, excluding the output file if specified
    let input_paths: Vec<PathBuf> = args.iter().take(args.len() - 1).map(PathBuf::from).collect();

    // Check that input files exist
    for path in &input_paths {
        if !path.exists() {
            eprintln!("Error: file not found: {:?}", path);
            return Ok(());
        }
    }

    // Open output file for writing
    let output_path = PathBuf::from(output_file_name);
    let mut output_file = fs::File::create(&output_path)?;

    // Combine files
    for path in &input_paths {
        let mut input_file = fs::File::open(&path)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;

        // Write input data directly to output file
        output_file.write_all(&input_data)?;
    }

    println!("Output file written to {:?}", output_path);

    Ok(())
}
