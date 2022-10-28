use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional string arg
    #[arg(short, long)]
    log_file: Option<String>,
    /// Number arg with default value
    #[arg(short, long, default_value_t = 1)]
    jobs: u8,
    /// Mandatory path to a file
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    if let Some(log_file) = args.log_file {
        println!("log_file: {}", log_file);
    }
    println!("{} jobs", args.jobs);
    if args.path.exists() {
        println!("{} exists", args.path.display());
    } else {
        println!("{} does not exist", args.path.display());
    }
}
