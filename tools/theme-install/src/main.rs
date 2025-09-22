use clap::Parser;
use moss_theme::conversion::convert_theme_json_to_css;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input theme json
    #[arg(short, long)]
    input_path: PathBuf,
    /// Path to the output theme css
    #[arg(short, long)]
    output_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let json = std::fs::read_to_string(&args.input_path).unwrap();
    let css = convert_theme_json_to_css(&json).unwrap();

    std::fs::write(&args.output_path, css.as_bytes()).unwrap();
}
