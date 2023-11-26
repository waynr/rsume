use clap::Parser;
use tracing_subscriber::EnvFilter;

mod errors;
mod generator;
mod resume;
mod templates;
mod themes;

use errors::Error;
use generator::Generator;
use generator::GeneratorParams;

#[derive(Parser)]
struct Cli {
    #[clap(flatten)]
    generator_params: GeneratorParams,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let level = match cli.debug {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive("off".parse()?)
        .add_directive(format!("resume_generator={0}", level.as_str()).parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    let generator = Generator::try_from(&cli.generator_params)?;
    generator.generate()?;

    Ok(())
}
