use std::path::PathBuf;
use url::Url;

use super::config_object::Config;
pub struct ConfigBuilder {
    credentials_file: Option<PathBuf>,
    profile: Option<String>,
    bucket: Option<Url>,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            credentials_file: None,
            profile: None,
            bucket: None,
        }
    }

    pub fn credentials_file(mut self, credentials_file: PathBuf) -> ConfigBuilder {
        self.credentials_file = Some(credentials_file);
        self
    }

    pub fn profile(mut self, profile: String) -> ConfigBuilder {
        self.profile = Some(profile);
        self
    }

    pub fn bucket(mut self, bucket: Url) -> ConfigBuilder {
        self.bucket = Some(bucket);
        self
    }

    pub fn build(self) -> Config {
        Config {
            credentials_file: self.credentials_file,
            profile: self.profile,
            bucket: self.bucket,
        }
    }
}
