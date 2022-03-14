use clap::Parser;
#[derive(Parser)]
struct Cli{
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len(){
        let file = archive.by_index(i).unwrap();
        println!("{}",file.name());
    }
}
