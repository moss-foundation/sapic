use clap::Parser;
use std::path::PathBuf;

use crate::{conversion::convert_theme_json_to_css, models::types::Theme};

mod conversion;
mod models;

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

    let rdr = std::io::BufReader::new(std::fs::File::open(&args.input_path).unwrap());
    let theme: Theme = serde_json::from_reader(rdr).unwrap();
    let css = convert_theme_json_to_css(theme);

    std::fs::write(&args.output_path, css.as_bytes()).unwrap();
}
