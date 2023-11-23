use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde::Serialize;
use serde::Deserialize;
use clap::Parser;
use json_resume::Resume as JsonResume;
use tempdir::TempDir;
use typst_cli::args::CompileCommand;
use typst_cli::compile;

use super::errors::Error;
use super::templates;
use super::themes::{self, Theme};

#[derive(Serialize, Deserialize)]
pub struct Resume {
    #[serde(flatten)]
    json_resume: JsonResume
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    description: Option<String>,
    industry_experience: Option<String>,
    education: Option<String>,
    interests: Vec<String>,
}

pub struct Generator {
    pub typst_source: String,
    pub resume: Resume,
    pub theme: Theme,

    pub output_file: Option<PathBuf>,
}

#[derive(Clone, Parser)]
pub struct GeneratorParams {
    #[arg(short, long)]
    typst_source: Option<PathBuf>,

    #[arg(long)]
    theme_file: Option<PathBuf>,

    data_file: PathBuf,

    output_file: Option<PathBuf>,
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

        let theme = if let Some(pb) = &params.theme_file {
            Theme::try_from(pb)?
        } else {
            serde_yaml::from_str(themes::DEFAULT)?
        };

        let mut data_file = File::open(&params.data_file)?;
        let mut yaml = String::new();
        data_file.read_to_string(&mut yaml)?;
        let resume: Resume = serde_yaml::from_str(yaml.as_str())?;

        Ok(Self {
            typst_source,
            resume,
            theme,
            output_file: params.output_file.clone(),
        })
    }
}

impl Generator {
    pub fn generate(&self) -> Result<(), Error> {
        let tmp_dir = TempDir::new("resume-generator")?;

        let typst_source_path = tmp_dir.path().join("rendered-resume.typ");
        let mut typst_file = File::create(&typst_source_path)?;
        typst_file.write_all(self.typst_source.as_bytes())?;

        let theme_str = serde_yaml::to_string(&self.theme)?;
        let theme_path = tmp_dir.path().join("theme.yaml");
        let mut theme_file = File::create(&theme_path)?;
        theme_file.write_all(theme_str.as_bytes())?;

        let data_str = serde_yaml::to_string(&self.resume)?;
        let data_path = tmp_dir.path().join("data.yaml");
        let mut data_file = File::create(&data_path)?;
        data_file.write_all(data_str.as_bytes())?;

        let mut cmd = CompileCommand::default();
        cmd.common.input = typst_source_path;
        cmd.output = self.output_file.clone();

        compile::compile(cmd).map_err(Error::TypstEcoStringError)?;
        Ok(())
    }
}
