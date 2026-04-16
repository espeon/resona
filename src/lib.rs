use nanorand::ChaCha;

use crate::utils::{err::SonaError, generate_name::generate_name};

mod utils;

#[derive(Clone, Debug)]
pub struct Config {
    pub min: usize,
    pub max: usize,
}

impl Config {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { min: 3, max: 8 }
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    min: Option<usize>,
    max: Option<usize>,
}

impl ConfigBuilder {
    pub fn min(mut self, min: usize) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    pub fn build(self) -> Config {
        Config {
            min: self.min.unwrap_or(3),
            max: self.max.unwrap_or(8),
        }
    }
}

pub fn sona(config: &Config) -> Result<String, SonaError> {
    let mut rng = ChaCha::new();
    generate_name(&mut rng, config)
}

pub fn sona_with_rng(rng: &mut ChaCha<8>, config: &Config) -> Result<String, SonaError> {
    generate_name(rng, config)
}
