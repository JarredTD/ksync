use std::path::{Path, PathBuf};

use ini::Ini;
use url::Url;

use super::config_builder::ConfigBuilder;
use crate::constants::{
    BUCKET_KEY, CREDENTIALS_FILE_KEY, CREDENTIALS_SECTION_NAME, PROFILE_KEY, S3_SECTION_NAME,
};

pub struct Config {
    pub(super) credentials_file: Option<PathBuf>,
    pub(super) profile: Option<String>,
    pub(super) bucket: Option<Url>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn write(self, file_path: &Path) -> Result<(), std::io::Error> {
        let mut config_file = Ini::new();

        if let (Some(credentials_file), Some(profile)) = (self.credentials_file, self.profile) {
            if let Some(credentials_path_str) = credentials_file.canonicalize()?.to_str() {
                config_file
                    .with_section(Some(CREDENTIALS_SECTION_NAME))
                    .set(CREDENTIALS_FILE_KEY, credentials_path_str)
                    .set(PROFILE_KEY, profile.as_str());
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid UTF-8 path",
                ));
            }
        }

        if let Some(bucket_url) = self.bucket {
            config_file
                .with_section(Some(S3_SECTION_NAME))
                .set(BUCKET_KEY, bucket_url.as_str());
        }

        config_file.write_to_file(file_path)?;

        Ok(())
    }

    pub fn read(file_path: &Path) -> Config {
        if let Ok(config_file) = Ini::load_from_file(file_path) {
            if let (Some(credentials_section), Some(s3_section)) = (
                config_file.section(Some(CREDENTIALS_SECTION_NAME)),
                config_file.section(Some(S3_SECTION_NAME)),
            ) {
                if let (Some(credentials_file), Some(profile), Some(bucket_url)) = (
                    credentials_section.get(CREDENTIALS_FILE_KEY),
                    credentials_section.get(PROFILE_KEY),
                    s3_section.get(BUCKET_KEY),
                ) {
                    if let Ok(bucket_url_parse) = Url::parse(bucket_url) {
                        return Config::builder()
                            .credentials_file(PathBuf::from(credentials_file))
                            .profile(String::from(profile))
                            .bucket(bucket_url_parse)
                            .build();
                    }
                }
            }
        }

        eprintln!("error reading config file {}", file_path.display());
        Config::builder().build()
    }
}
