use clap::Parser;
use scroll_proving_agent::config::SnarkifyConfig;
use scroll_proving_agent::prover::SnarkifyProver;
use scroll_proving_sdk::{prover::ProverBuilder, utils::init_tracing};

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
struct Args {
    /// Path to the configuration file in JSON format.
    /// Regarding the JSON format, please refer to the README.md for the Configuration section in
    /// https://github.com/snarkify/scroll-proving-agent
    #[arg(long = "config", default_value = "config.json")]
    config_file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let args = Args::parse();
    let config: SnarkifyConfig = SnarkifyConfig::from_file(args.config_file)?;
    let snarkify_prover = SnarkifyProver::new(config.clone());
    let prover = ProverBuilder::new(config.sdk_config, snarkify_prover)
        .build()
        .await?;
    prover.run().await;
    Ok(())
}
