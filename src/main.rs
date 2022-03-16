

use std::io::Read;

use clap::StructOpt;
use cli::cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    match &args.command{
        Commands::Read {page: None}=>{
            
        },
        Commands::Read { page: Some(page)}=>{
            let mut contents = Vec::new();
            let mut zip_file = archive.by_index(*page).unwrap();
            zip_file.read_to_end(&mut contents).unwrap();
            let contents = &contents[..];
            println!("{}",std::str::from_utf8(contents).unwrap());
        },
        Commands::Extract { path }=>{
            archive.extract(path).unwrap();
        }
    }
}

