use std::fs::copy;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;
use json_resume::Resume;
use tempdir::TempDir;
use typst_cli::args::CompileCommand;
use typst_cli::compile;

use super::errors::Error;
use super::templates;

pub struct Generator {
    typst_source: String,
    resume: Resume,
    theme_file: PathBuf,
}

#[derive(Clone, Parser)]
pub struct GeneratorParams {
    #[arg(short, long)]
    typst_source: Option<PathBuf>,

    data_file: PathBuf,

    theme_file: PathBuf,
}

impl TryFrom<&GeneratorParams> for Generator {
    type Error = Error;

    fn try_from(params: &GeneratorParams) -> Result<Self, Error> {
        let typst_source = if let Some(s) = params
            .typst_source
            .as_ref()
            .map(|pb| -> Result<String, Error> {
                let mut f = File::open(&pb)?;
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

        let mut data_file = File::open(&params.data_file)?;
        let mut yaml = String::new();
        data_file.read_to_string(&mut yaml)?;
        let resume: Resume = serde_yaml::from_str(yaml.as_str())?;

        Ok(Self {
            typst_source,
            resume,
            theme_file: params.theme_file.clone(),
        })
    }
}

impl Generator {
    pub fn generate(&self) -> Result<(), Error> {
        let tmp_dir = TempDir::new("resume-generator")?;
        let typst_source_path = tmp_dir.path().join("rendered-resume.typ");

        let mut tmp_file = File::create(&typst_source_path)?;
        tmp_file.write_all(self.typst_source.as_bytes())?;

        let theme_path = tmp_dir.path().join("theme.yaml");
        copy(&self.theme_file, theme_path)?;

        let resume_str = serde_yaml::to_string(&self.resume)?;
        let data_path = tmp_dir.path().join("data.yaml");
        let mut data_file = File::create(&data_path)?;
        data_file.write_all(resume_str.as_bytes())?;

        let mut cmd = CompileCommand::default();
        cmd.common.input = typst_source_path;
        cmd.output = Some(PathBuf::from("./resume.pdf"));

        compile::compile(cmd).map_err(Error::TypstEcoStringError)?;
        Ok(())
    }
}
