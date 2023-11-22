use clap::Parser;

mod errors;
mod generator;
mod templates;

use errors::Error;
use generator::GeneratorParams;
use generator::Generator;

#[derive(Parser)]
struct Cli {
    #[clap(flatten)]
    generator_params: GeneratorParams,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let generator = Generator::try_from(&cli.generator_params)?;
    generator.generate()?;

    Ok(())
}
