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

    pub fn update(&mut self, key: String, value: String) {
        match key.as_str() {
            "credentials_file" => {
                log::trace!("Setting 'credentials_file' to {}", value);
                self.credentials_file = Some(PathBuf::from(value));
            }
            "profile" => {
                log::trace!("Setting 'profile' to {}", value);
                self.profile = Some(value);
            }
            "bucket" => {
                if let Ok(bucket_url) = Url::parse(&value) {
                    log::trace!("Setting 'bucket' to {}", value);
                    self.bucket = Some(bucket_url);
                } else {
                    log::error!("invalid bucket URL: {}", value);
                }
            }
            _ => {
                log::error!("unknown configuration key: {}", key);
            }
        }
    }

    pub fn write(self, file_path: &Path) -> Result<(), std::io::Error> {
        let mut config_file = Ini::new();
    
        if let Some(credentials_file) = self.credentials_file {
            if let Some(credentials_path_str) = credentials_file.canonicalize()?.to_str() {
                config_file
                    .with_section(Some(CREDENTIALS_SECTION_NAME))
                    .set(CREDENTIALS_FILE_KEY, credentials_path_str);
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid UTF-8 path",
                ));
            }
        }
    
        if let Some(profile) = self.profile {
            config_file
                .with_section(Some(CREDENTIALS_SECTION_NAME))
                .set(PROFILE_KEY, profile.as_str());
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
        // Initialize builder
        let mut config_builder = Config::builder();
    
        // Try to load the INI file
        match Ini::load_from_file(file_path) {
            Ok(config_file) => {
                let credentials_section = config_file.section(Some(CREDENTIALS_SECTION_NAME));
                let s3_section = config_file.section(Some(S3_SECTION_NAME));
    
                if let Some(credentials_section) = credentials_section {
                    if let Some(credentials_file) = credentials_section.get(CREDENTIALS_FILE_KEY) {
                        config_builder = config_builder.credentials_file(PathBuf::from(credentials_file));
                    } else {
                        log::info!("Missing credentials file in INI.");
                    }
    
                    if let Some(profile) = credentials_section.get(PROFILE_KEY) {
                        config_builder = config_builder.profile(String::from(profile));
                    } else {
                        log::info!("Missing profile in INI.");
                    }
                } else {
                    log::info!("Missing credentials section in INI.");
                }
    
                if let Some(s3_section) = s3_section {
                    if let Some(bucket_url) = s3_section.get(BUCKET_KEY) {
                        match Url::parse(bucket_url) {
                            Ok(bucket_url_parse) => {
                                config_builder = config_builder.bucket(bucket_url_parse);
                            }
                            Err(e) => {
                                log::error!("Invalid bucket URL: {}", e);
                            }
                        }
                    } else {
                        log::info!("Missing bucket URL in INI.");
                    }
                } else {
                    log::info!("Missing S3 section in INI.");
                }
            }
            Err(e) => {
                log::error!("Failed to load configuration file: {}", e);
            }
        }
    
        config_builder.build()
    }
}
