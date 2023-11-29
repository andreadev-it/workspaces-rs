use clap::{Parser, Subcommand};

mod config;
mod workspaces;
use config::Config;
use workspaces::{list_workspaces, add_workspace, remove_workspace, search};

#[derive(Parser, Debug)]
#[command(name="workspaces")]
struct Cli {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    Add { name: String },
    Remove { name: String },
    List
}


fn main() {
    let cli = Cli::parse();

    let mut config = Config::build();

    match &cli.command {
        Some(Subcommands::Add { name }) => {
            add_workspace(name, &mut config);
        },
        Some(Subcommands::Remove { name }) => {
            remove_workspace(name, &mut config);
        },
        Some(Subcommands::List) => {
            list_workspaces(&config);
        }
        None => {
            search(&config);
        }
    }
}

