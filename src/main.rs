use clap::{Parser, Subcommand};
use directories::ProjectDirs;
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
        /// Path of the directory to remove
        #[arg(help = "The directory path to remove")]
        path: String,
    },
    /// Views details of a specific directory shortcut
    View {
        /// Name of the shortcut to view
        #[arg(help = "The shortcut name to view")]
        name: String,
    },
}

fn main() {
    let args = Args::parse();

    // Handle optional subcommands
    match (&args.dirname, &args.command) {
        (Some(dirname), None) => {
            // If only a directory name is provided, treat it as a request to navigate
            println!("Changing to directory: {}", dirname);
            // Here you would typically trigger some action to handle this
        }
        (_, Some(Commands::Add { name, path })) => {
            println!("Adding new directory '{}': '{}'", name, path);
            // Code to add the directory shortcut
            helper::add_to_file(name, path)
        }
        (_, Some(Commands::Remove { name, path })) => {
            println!("Removing directory '{}': '{}'", name, path);
            // Code to remove the directory shortcut
        }
        (_, Some(Commands::View { name })) => {
            println!("Viewing details for directory '{}'", name);
            // Code to display details of the directory shortcut
        }
        (None, None) => {
            println!("No directory specified and no command provided. Please specify a directory or use a subcommand.");
        }
    }
}
