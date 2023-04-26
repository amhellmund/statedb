use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use statedb::{initialize_database,launch_database,open_console};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new StateDB
    Init(InitArgs),

    /// Launches the StateDB server
    Launch(LaunchArgs),

    /// Opens a StateDB console
    Console(ConsoleArgs),
}

#[derive(Args)]
struct InitArgs {
    /// The directory to initialize the new StateDB instance into.
    #[arg(short, long)]
    database_dir: PathBuf,

    /// The configuration file to initialize the StateDB instance with.
    #[arg(short, long)]
    config_file: PathBuf,
}

#[derive(Args)]
struct LaunchArgs {
    /// The StateDB directory to start the server from.
    #[arg(short, long)]
    database_dir: PathBuf,

    /// The IP address of the server to listen on.
    #[arg(short, long)]
    address: String,

    /// The TCP/IP port of the server to listen on.
    #[arg(short, long)]
    port: i16,
}

#[derive(Args)]
struct ConsoleArgs {
    /// The TCP/IP port of the running server.
    #[arg(short, long)]
    port: i16,

    /// The name of the user.
    #[arg(short, long)]
    user: String,

    /// The password of the user
    #[arg(short, long)]
    password: String,
}

fn main () {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Init(args) => {
            initialize_database(&args.database_dir, &args.config_file);
        },
        Commands::Launch(args) => {
            launch_database(&args.database_dir, &args.address, args.port);
        },
        Commands::Console(args) => {
            open_console(args.port, &args.user, &args.password);
        },
    }
}
