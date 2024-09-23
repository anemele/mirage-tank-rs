use clap::Parser;

/// A CLI tool to make mirage tank images
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// path to top image
    pub top_image: String,
    /// path to bottom image
    pub bottom_image: String,
    /// path to output image
    #[arg(short, long, default_value = DEFAULT_OUTPUT)]
    pub output: String,
}

pub const DEFAULT_OUTPUT: &str = "output.png";
