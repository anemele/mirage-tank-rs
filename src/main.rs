use clap::Parser;

mod cli;
mod de;
mod img;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::Make {
            top_image,
            bottom_image,
            output,
        } => {
            img::make(&top_image, &bottom_image, &output)?;
        }
        Cli::De { input } => {
            de::operate(&input)?;
        }
    }
    Ok(())
}
