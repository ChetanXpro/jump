use std::process;

use clap::{Parser, Subcommand};
mod helper;
mod storage;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "go", version = "1.0", about = "Manages and navigates directories", long_about = None)]
struct Args {
    #[arg(help = "The directory name to navigate to or manage", required = false)]
    dirname: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds a new directory shortcut
    Add {
        /// Name of the shortcut to add
        #[arg(help = "The shortcut name")]
        name: String,
        /// Path of the directory to add
        #[arg(help = "The directory path to add")]
        path: String,
    },
    /// Removes an existing directory shortcut
    Remove {
        /// Name of the shortcut to remove
        #[arg(help = "The shortcut name")]
        name: String,
    },
    /// Views details of a specific directory shortcut
    View {
        /// Name of the shortcut to view
        #[arg(help = "The optional shortcut name to view", required = false)]
        name: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    // Handle optional subcommands
    match (&args.dirname, &args.command) {
        (Some(dirname), None) => {
            match helper::read_from_file(dirname.to_owned()) {
                Ok(path_buf) => {
                    // Ensure only the path is printed to stdout for shell consumption
                    println!("{}", path_buf.display());
                }
                Err(e) => {
                    eprintln!("Error: {}", e); // Print errors to stderr
                    process::exit(1);
                }
            }
        }
        (_, Some(Commands::Add { name, path })) => {
            // println!("Adding new directory '{}': '{}'", name, path);
            // Code to add the directory shortcut
            helper::add_to_file(name, path)
        }
        (_, Some(Commands::Remove { name })) => {
            // println!("Removing directory '{}': '{}'", name, path);
            storage::remove_from_json(name.to_owned()).unwrap();
            // Code to remove the directory shortcut
        }
        (_, Some(Commands::View { name })) => {
            // match helper::read_from_file(name.to_owned()) {
            //     Ok(path_buf) => {
            //         // Ensure only the path is printed to stdout for shell consumption
            //         println!("Path: {}", path_buf.display());
            //     }
            //     Err(e) => {
            //         eprintln!("Error: {}", e); // Print errors to stderr
            //         process::exit(1);
            //     }
            // }
            match name {
                Some(name) => match helper::read_from_file(name.to_owned()) {
                    Ok(path_buf) => println!("Path: {}", path_buf.display()),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                },
                None => {
                    // Code to list all entries
                    storage::list_all_names().unwrap();
                }
            }
        }
        (None, None) => {
            println!("No directory specified and no command provided. Please specify a directory or use a subcommand.");
        }
    }
}
