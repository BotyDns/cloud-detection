//! this module provides tools to load the configuration files for the cli utility.

use confy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// The configuration for Landsat images.
pub struct LandsatConfig {
    /// The ban
    pub green_band_index: isize,
    pub red_band_index: isize,
}

#[derive(Serialize, Deserialize)]
/// The configuration for Sentinel images
pub struct SentinelConfig {
    pub green_band_index: isize,
    pub red_band_index: isize,
}

#[derive(Serialize, Deserialize)]
/// The main configuration struct
pub struct CloudDetectionConfig {
    pub landsat: LandsatConfig,
    pub sentinel: SentinelConfig,
}

impl ::std::default::Default for CloudDetectionConfig {
    fn default() -> Self {
        Self {
            landsat: LandsatConfig {
                green_band_index: 3,
                red_band_index: 4,
            },
            sentinel: SentinelConfig {
                green_band_index: 3,
                red_band_index: 4,
            },
        }
    }
}

/// Loads the config file. If config_path is None, a default configuration is returned.
pub fn load_config(
    config_path: &Option<String>,
) -> Result<CloudDetectionConfig, confy::ConfyError> {
    match config_path {
        None => Ok(CloudDetectionConfig::default()),
        Some(path) => confy::load_path(path),
    }
}
