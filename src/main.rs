use std::fs;
use clap::Parser;
use goblin::Object;

/// Puts all the files in a directory into a vector. On success returns None, else returns the std::io::error 
/// from io::Result.
/// 
/// # Arguments
///
/// * `directory_path` - The path which to collect all files in.
/// * `total_paths` - The vector which is populated with the files in the directory.
fn collect_all_files(directory_path: &str, total_paths: &mut Vec<std::path::PathBuf>) -> Option<std::io::Error> {

    match fs::read_dir(directory_path) {
        Ok(paths) => {
            for path in paths {
                if let Ok(path) = path {
                    total_paths.push(path.path());
                }
            }
        },
        Err(error) => {
            return Some(error);
        }
    }
    return None;
}

/// A program which can query a directory of files, find the binaries, and search for a specified Win API import.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Directory where to query for PE file imports
    #[clap(value_name = "Directory")]
    input_directory: String,
    /// WinAPI import to search for
    #[clap(value_name = "Import Name")]
    import_name: String,
    /// Print extra output while parsing
    #[clap(short, long)]
    verbose: bool
}

fn main() {

    let args = Args::parse();
    let api_query = args.import_name;

    let mut total_paths : Vec<std::path::PathBuf> = vec![];

    if let Some(error) = collect_all_files("C:\\Windows\\System32\\Drivers", &mut total_paths) {
        panic!("[!] Error: {error}");
    }

    for path in &total_paths {
        if let Ok(buffer) = fs::read(path) {
            let object = Object::parse(&buffer);
            match object {
                Ok(object) => {
                    match object {
                        Object::PE(pe) => {
                            for import in &pe.imports {
                                if import.name.eq(&api_query) {
                                    println!("[i] Found API ({}) match in {}", api_query, path.display());
                                }
                            }  
                        },
                        Object::Elf(_) => (),
                        Object::Mach(_) => (),
                        Object::Archive(_) => (),
                        Object::Unknown(_) => ()
                    }
                },
                Err(error) => {
                    println!("[!] Error on {}: {}. Skipping...", path.display(), error);
                }
            }
        }
    }       
}
