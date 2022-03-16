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
pub mod xml_parser;