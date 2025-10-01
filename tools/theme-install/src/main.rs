use clap::Parser;
use moss_app::theme::install::install_theme;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input theme json
    #[arg(short, long)]
    input_path: PathBuf,
    /// Path to the rego policy used for validation
    #[arg(short, long)]
    policy_path: PathBuf,
    /// Path to the output themes dir
    #[arg(short, long)]
    output_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    install_theme(&args.input_path, &args.policy_path, &args.output_path).unwrap();
}
