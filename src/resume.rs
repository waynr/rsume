use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use json_resume::Resume as JsonResume;
use serde::Deserialize;
use serde::Serialize;

use super::errors::Error;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    description: Option<String>,
    industry_experience: Option<String>,
    education: Option<String>,
    interests: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Resume {
    /// Extension to json_resume schema for alternative summary style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) summary: Option<Summary>,

    #[serde(flatten)]
    pub(crate) json_resume: JsonResume,
}

enum SupportedExtensions {
    Yaml,
    Json,
    Unknown,
}

const YAML_EXTENSIONS: [&'static str; 2] = ["yaml", "yml"];
const JSON_EXTENSIONS: [&'static str; 1] = ["json"];

impl From<&OsStr> for SupportedExtensions {
    fn from(s: &OsStr) -> Self {
        let s = <&str>::try_from(s).unwrap();
        if YAML_EXTENSIONS.contains(&s) {
            SupportedExtensions::Yaml
        } else if JSON_EXTENSIONS.contains(&s) {
            SupportedExtensions::Json
        } else {
            SupportedExtensions::Unknown
        }
    }
}

impl TryFrom<&PathBuf> for Resume {
    type Error = Error;

    fn try_from(pb: &PathBuf) -> Result<Self, Self::Error> {
        let ext = match pb.extension() {
            Some(s) => SupportedExtensions::from(s),
            None => {
                eprintln!("no file extension found in {pb:?}, will treat as yaml");
                SupportedExtensions::Yaml
            }
        };

        let mut data_file = File::open(&pb)?;
        let mut deserialized = String::new();
        data_file.read_to_string(&mut deserialized)?;

        let resume = match ext {
            SupportedExtensions::Yaml | SupportedExtensions::Unknown => {
                serde_yaml::from_str(deserialized.as_str())?
            }
            SupportedExtensions::Json => serde_json::from_str(deserialized.as_str())?,
        };

        Ok(resume)
    }
}

#[cfg(test)]
mod test {
    use super::super::errors::Error;
    use super::*;

    #[test]
    fn validate_from_yaml_ext() -> Result<(), Error> {
        let pb = PathBuf::from("./testdata/sample.resume.yaml".to_string());
        assert!(pb.exists());

        let _ = Resume::try_from(&pb)?;

        Ok(())
    }

    #[test]
    fn validate_json_yaml_testdata_equivalence() -> Result<(), Error> {
        let pb_yaml = PathBuf::from("./testdata/sample.resume.yaml".to_string());
        assert!(pb_yaml.exists());

        let pb_json = PathBuf::from("./testdata/sample.resume.json".to_string());
        assert!(pb_json.exists());

        let resume_yaml = Resume::try_from(&pb_yaml)?;
        let resume_json = Resume::try_from(&pb_json)?;

        assert_eq!(resume_yaml, resume_json);

        Ok(())
    }
}
