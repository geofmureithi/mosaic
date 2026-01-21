use std::path::PathBuf;

use anyhow::Result;
use figment::{Figment, providers::{Env, Format, Toml}};
use serde::{Deserialize, Serialize};

use crate::Cli;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// RPC URL for Solana cluster
    #[serde(default = "default_rpc_url")]
    rpc_url: String,

    /// Program ID
    program_id: Option<String>,

    /// Default payer keypair path
    payer_keypair: Option<PathBuf>,

    /// Destination program ID
    destination_program: Option<String>,
}

fn default_rpc_url() -> String {
    "https://api.mainnet-beta.solana.com".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: default_rpc_url(),
            program_id: None,
            payer_keypair: None,
            destination_program: None,
        }
    }
}

pub fn load_config(config_path: Option<&PathBuf>) -> Result<Config> {
    let mut figment = Figment::new();

    // Load from config file if provided
    if let Some(path) = config_path {
        figment = figment.merge(Toml::file(path));
    } else {
        // Try default config locations
        figment = figment
            .merge(Toml::file("Mosaic.toml"));
    }

    // Override with environment variables
    figment = figment.merge(Env::prefixed("MOSAIC_"));

    Ok(figment.extract()?)
}

pub fn merge_cli_config(config: &mut Config, cli: &Cli) {
    if let Some(rpc_url) = &cli.rpc_url {
        config.rpc_url = rpc_url.clone();
    }
    if let Some(program_id) = &cli.program_id {
        config.program_id = Some(program_id.clone());
    }
}
