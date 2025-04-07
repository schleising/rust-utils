use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, crate_name, crate_version};

#[derive(Parser, Debug)]
#[command(name = "remove-folders")]
#[command(about = "Remove folders from a directory")]
#[command(version)]
struct Args {
    /// The path to the directory to search
    #[arg()]
    base_folder: String,

    /// The name of the folder to remove
    #[arg()]
    folder: String,
}

fn walk_dir<P: AsRef<Path>>(path: P, folder_to_remove: &str) -> io::Result<Vec<PathBuf>> {
    let path = path.as_ref();
    let mut paths: Vec<PathBuf> = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    // println!("Found folder: {}", name.to_string_lossy());
                    if name == folder_to_remove {
                        // Add the folder to the list of paths to remove
                        paths.push(path.clone());

                        // Print the folder that will be removed
                        println!("Found folder to remove: {}", path.display());

                        // Once we find a folder to remove, we can stop searching this directory
                        return Ok(paths);
                    }
                }

                // Recursively search the subdirectory
                let sub_paths = walk_dir(&path, folder_to_remove)?;

                // Add the subdirectory paths to the list of paths to remove
                paths.extend(sub_paths);
            }
        }
    }
    Ok(paths)
}

fn main() {
    let args = Args::parse();

    println!("{}", crate_name!());
    println!("Version: {}", crate_version!());

    // Create a PathBuf from the base folder string
    let base_folder: PathBuf = args.base_folder.into();

    // Canonicalise the base folder
    let base_folder = match base_folder.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Print the base folder and the folder to remove
    println!(
        "Searching {} for folders named: {}",
        base_folder.display(),
        args.folder
    );
    println!();

    // Check if the base folder exists
    if !base_folder.exists() {
        eprintln!("Error: {} does not exist.", base_folder.display());
        return;
    }

    // Check if the base folder is a directory
    if !base_folder.is_dir() {
        eprintln!("Error: {} is not a directory.", base_folder.display());
        return;
    }

    // Get the list of paths to remove
    let paths = match walk_dir(&base_folder, &args.folder) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Print the folders which will be removed
    if paths.is_empty() {
        println!("No folders found to remove, exiting.");
        return;
    }
    println!("Found {} folders to remove:", paths.len());

    // Ask the user for confirmation
    println!();
    print!("Are you sure you want to remove these folders (y/n)?: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_lowercase();
    if input != "y" {
        println!();
        println!("Exiting without removing folders.");
        return;
    }
    println!();
    println!("Removing folders...");

    // Remove the folders
    for (count, path) in paths.iter().enumerate() {
        println!(
            "Removing folder {:2} of {:2}: {}",
            count + 1,
            paths.len(),
            path.display()
        );

        if let Err(e) = fs::remove_dir_all(path) {
            eprintln!("Error removing folder {}: {}", path.display(), e);
        } else {
            println!("Removed folder: {}", path.display());
        }
    }
    println!("Done.");
}
