use anyhow::Result;

use colored::Colorize;
use drand_core::{
    beacon::RandomnessBeacon,
    chain::{self, ChainClient, ChainOptions, ChainVerification},
    http_chain_client::HttpChainClient,
};

use crate::{
    config::{self, ConfigChain},
    print::{print_with_format, Format, Print},
};

impl Print for RandomnessBeacon {
    fn pretty(&self) -> Result<String> {
        Ok(format!(
            r"{: <10}: {}
{: <10}: {}
{: <10}: {}",
            "Round".bold(),
            self.round(),
            "Randomness".bold(),
            hex::encode(self.randomness()),
            "Signature".bold(),
            hex::encode(self.signature()),
        ))
    }

    fn json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

pub async fn rand(
    _cfg: &config::Local,
    format: Format,
    chain: ConfigChain,
    beacon: Option<u64>,
    verify: bool,
) -> Result<String> {
    let chain = chain::Chain::new(&chain.url());
    let info = chain.info().await?;

    let client = HttpChainClient::new(
        chain,
        Some(ChainOptions::new(
            verify,
            true,
            Some(ChainVerification::new(
                Some(info.hash()),
                Some(info.public_key()),
            )),
        )),
    );

    let beacon = match beacon {
        Some(round) => client.get(round).await?,
        None => client.latest().await?,
    };

    print_with_format(beacon, format)
}