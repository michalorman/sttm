use clap::{Parser, Subcommand};
use sttm::gui::run;
use tuikit::error::Result;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>
}

#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    Add {
        description: Vec<String>
    },

    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(ref command) = &cli.command {
        match command {
            Command::Add { description } => println!("adding task: '{}'", description.join(" ")),

            Command::List => println!("list tasks"),
        }

        Ok(())
    } else {
        run()
    }
}
