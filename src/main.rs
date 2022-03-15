

use std::io::Read;

use clap::Parser;
fn main() {
    let args = cli::Cli::parse();
    let file = std::fs::File::open(&args.path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    match &args.command{
        cli::Commands::Read {page: None}=>{
            
        },
        cli::Commands::Read { page: Some(page)}=>{
            let mut contents = Vec::new();
            let mut zip_file = archive.by_index(*page).unwrap();
            zip_file.read_to_end(&mut contents).unwrap();
            let contents = &contents[..];
            println!("{}",std::str::from_utf8(contents).unwrap());
        },
        cli::Commands::Extract { path }=>{
            archive.extract(path).unwrap();
        }
    }
}
pub mod cli{
    use clap::{Parser, Subcommand};
    #[derive(Parser)]
    pub struct Cli{
        #[clap(subcommand)]
        pub command: Commands,
        #[clap(parse(from_os_str))]
        pub path: std::path::PathBuf,
    }
    #[derive(Subcommand)]
    pub enum Commands{
        Read{
            #[clap(short,long)]
            page: Option<usize>,
        },
        Extract{
            #[clap(parse(from_os_str))]
            path: std::path::PathBuf,
        }
    }
}
