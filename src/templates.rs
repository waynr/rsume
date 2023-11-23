pub const DEFAULT: &'static str = include_str!("../templates/general-purpose.typ");

#[cfg(test)]
mod test {
    use super::super::errors::Error;
    use super::super::generator::Generator;
    use super::super::themes;
    use super::*;

    const SAMPLE_RESUME: &'static str = include_str!("../testdata/sample.resume.json");

    #[test]
    fn test_default() -> Result<(), Error> {
        let g = Generator {
            typst_source: DEFAULT.to_string(),
            resume: serde_json::from_str(SAMPLE_RESUME)?,
            theme: themes::DEFAULT.try_into()?,
            output_file: None,
        };
        g.generate()?;
        Ok(())
    }
}
