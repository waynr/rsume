use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;
use merge_struct::merge;
use tempdir::TempDir;
use tera::{Context, Tera};
use typst_cli::args::CompileCommand;
use typst_cli::compile;

use super::errors::{Error, Result};
use super::resume::Resume;
use super::templates;
use super::themes::{self, Theme};

pub struct Generator {
    pub typst_source: String,
    pub resume: Resume,
    pub theme: Theme,

    pub output_file: Option<PathBuf>,
}

#[derive(Clone, Parser)]
pub struct GeneratorParams {
    #[arg(short, long)]
    pub typst_source: Option<PathBuf>,

    #[arg(long)]
    pub theme_file: Option<PathBuf>,

    #[arg(short, long)]
    pub output_file: Option<PathBuf>,

    pub data_file: Vec<PathBuf>,
}

impl TryFrom<&GeneratorParams> for Generator {
    type Error = Error;

    fn try_from(params: &GeneratorParams) -> Result<Self> {
        let typst_source = if let Some(s) = params
            .typst_source
            .as_ref()
            .map(|pb| -> Result<String> {
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

        let resume: Resume = if params.data_file.len() > 1 {
            let base = Resume::try_from(&params.data_file[0])?;
            params
                .data_file
                .iter()
                .skip(1)
                .try_fold(base, |acc, data_path| -> Result<Resume> {
                    let overrides = Resume::try_from(data_path)?;
                    Ok(merge(&acc, &overrides)?)
                })?
        } else {
            Resume::try_from(&params.data_file[0])?
        };

        Ok(Self {
            typst_source,
            resume,
            theme,
            output_file: params.output_file.clone(),
        })
    }
}

impl Generator {
    pub fn generate(&self) -> Result<()> {
        let tmp_dir = TempDir::new("resume-generator")?;

        let skill_keywords = self
            .resume
            .json_resume
            .skills
            .iter()
            .map(|skill| skill.keywords.iter())
            .flatten()
            .map(|k| k.0.clone())
            .collect::<Vec<String>>();
        let mut ctx = Context::new();
        ctx.insert("keywords", &skill_keywords);

        let mut tera = Tera::default();
        tera.add_raw_template(
            "template",
            &self.typst_source,
        )?;
        let typst_source = tera.render("template", &ctx)?;
        tracing::debug!("{typst_source}");

        let typst_source_path = tmp_dir.path().join("rendered-resume.typ");
        let mut typst_file = File::create(&typst_source_path)?;
        typst_file.write_all(typst_source.as_bytes())?;

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

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_try_from_params() -> Result<()> {
        let params = GeneratorParams {
            typst_source: Some(PathBuf::from_str("./templates/general-purpose.typ").unwrap()),
            theme_file: Some(PathBuf::from_str("./themes/default.yaml").unwrap()),
            data_file: PathBuf::from_str("./testdata/sample.resume.yaml").unwrap(),
            output_file: None,
        };
        let g = Generator::try_from(&params)?;
        g.generate()?;
        Ok(())
    }
}
