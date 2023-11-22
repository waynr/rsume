use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::copy;

use clap::Parser;
use json_resume::Resume;
use tempdir::TempDir;
use typst_cli::args::CompileCommand;
use typst_cli::compile;

mod errors;
mod templates;

use errors::Error;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    template: Option<PathBuf>,

    #[clap(flatten)]
    template_parameters: TemplateParameters,
}

#[derive(Clone, Parser)]
struct TemplateParameters {
    data_file: PathBuf,

    theme_file: PathBuf,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let typst_source = if let Some(s) = cli
        .template
        .map(|pb| -> Result<String, Error> {
            let mut f = File::open(pb)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        })
        .transpose()?
    {
        s
    } else {
        templates::DEFAULT.to_string()
    };

    let tmp_dir = TempDir::new("resume-generator")?;
    let typst_source_path = tmp_dir.path().join("rendered-resume.typ");

    let mut tmp_file = File::create(&typst_source_path)?;
    tmp_file.write_all(typst_source.as_bytes())?;

    let theme_source_path = tmp_dir.path().join("theme.yaml");
    copy(cli.template_parameters.theme_file, theme_source_path)?;
    let data_source_path = tmp_dir.path().join("data.yaml");
    copy(cli.template_parameters.data_file, data_source_path)?;

    let mut cmd = CompileCommand::default();
    cmd.common.input = typst_source_path;
    cmd.output = Some(PathBuf::from("./resume.pdf"));

    compile::compile(cmd).map_err(Error::TypstEcoStringError)?;
    Ok(())
}
