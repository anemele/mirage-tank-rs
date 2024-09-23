use clap::Parser;

mod cli;
mod img;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    // dbg!(args);
    img::make(&args.top_image, &args.bottom_image, &args.output)?;
    Ok(())
}
