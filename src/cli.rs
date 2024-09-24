use clap::Parser;

/// A CLI tool for mirage tank images
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub enum Cli {
    /// Make a mirage tank image
    Make {
        /// path to top image
        top_image: String,
        /// path to bottom image
        bottom_image: String,
        /// path to output image
        #[arg(short, long, default_value = DEFAULT_OUTPUT)]
        output: String,
    },

    /// Seperate the top and bottom parts of a mirage tank image
    De {
        /// path to input image
        input: String,
    },
}

pub const DEFAULT_OUTPUT: &str = "output.png";
