use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use std::fmt;

use colored::*;

use grin_wallet::{WalletConfig};

use super::Result;

const WALLET713_CONFIG_PATH: &str = "wallet713.toml";

const GRIN_HOME: &str = ".grin";
const GRIN_NODE_API_SECRET_FILE: &str = ".api_secret";

const DEFAULT_CONFIG: &str = r#"
	wallet713_data_path = "wallet713_data"
	grinbox_domain = "grinbox.io"
	grinbox_port = 13420
	grinbox_private_key = ""
	grin_node_uri = "http://127.0.0.1:13413"
	grin_node_secret = ""
"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet713Config {
    pub wallet713_data_path: String,
    pub grinbox_domain: String,
    pub grinbox_port: u16,
    pub grinbox_private_key: String,
    pub grin_node_uri: String,
    pub grin_node_secret: Option<String>,
}

impl Wallet713Config {
    pub fn exists() -> bool {
        Path::new(WALLET713_CONFIG_PATH).exists()
    }

    pub fn from_file() -> Result<Wallet713Config> {
        let mut file = File::open(WALLET713_CONFIG_PATH)?;
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)?;
        let result: std::result::Result<Wallet713Config, toml::de::Error> = toml::from_str(&toml_str[..]);
        if result.is_err() {
            // try to load old version of config and convert to new version
            cli_message!("{}: trying to convert configuration from older version", "WARNING".bright_yellow());
            let config_v1: Wallet713ConfigV1 = toml::from_str(&toml_str[..])?;
            let mut config: Wallet713Config = toml::from_str(DEFAULT_CONFIG)?;
            config.wallet713_data_path = config_v1.wallet713_data_path;
            config.grinbox_private_key = config_v1.grinbox_private_key;
            config.grin_node_uri = config_v1.grin_node_uri;
            config.grin_node_secret = config_v1.grin_node_secret;
            cli_message!("{}: configuration coverted successfully", "INFO".bright_blue());
            Ok(config)
        } else {
            Ok(result.unwrap())
        }
    }

    pub fn default() -> Result<Wallet713Config> {
        let mut config: Wallet713Config = toml::from_str(DEFAULT_CONFIG)?;
        config.grin_node_secret = None;
        if let Some(mut home_path) = dirs::home_dir() {
            home_path.push(GRIN_HOME);
            home_path.push(GRIN_NODE_API_SECRET_FILE);
            let path_str = home_path.to_str().unwrap();
            let mut file = File::open(&path_str)?;
            let mut contents: String = String::new();
            file.read_to_string(&mut contents)?;
            config.grin_node_secret = Some(contents);
        };
        Ok(config)
    }

    pub fn to_file(&self) -> Result<()> {
        let toml_str = toml::to_string(&self)?;
        let mut f = File::create(WALLET713_CONFIG_PATH)?;
        f.write_all(toml_str.as_bytes())?;
        Ok(())
    }

    pub fn as_wallet_config(&self) -> Result<WalletConfig> {
        let mut wallet_config = WalletConfig::default();
        wallet_config.data_file_dir = self.wallet713_data_path.clone();
        wallet_config.check_node_api_http_addr = self.grin_node_uri.clone();
        Ok(wallet_config)
    }
}

impl fmt::Display for Wallet713Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wallet713_data_path={}\ngrinbox_domain={}\ngrinbox_port={}\ngrinbox_private_key={}\ngrin_node_uri={}\ngrin_node_secret={}",
               self.wallet713_data_path,
               self.grinbox_domain,
               self.grinbox_port,
               "{...}",
               self.grin_node_uri,
               "{...}")?;
        if self.grinbox_private_key.is_empty() {
            write!(f, "\n{}: grinbox keypair not set! consider using `config --generate-keys`", "WARNING".bright_yellow())?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet713ConfigV1 {
    pub wallet713_data_path: String,
    pub grinbox_uri: String,
    pub grinbox_private_key: String,
    pub grin_node_uri: String,
    pub grin_node_secret: Option<String>,
}
