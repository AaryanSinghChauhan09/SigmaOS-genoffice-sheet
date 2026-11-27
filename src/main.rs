#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "genoffice-sheet", about = "SigmaOS GenOffice spreadsheet")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("genoffice-sheet: no entries configured."),
        Commands::Status => println!("genoffice-sheet: ready."),
    }
}
