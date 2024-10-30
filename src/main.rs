use clap::Parser;
use scroll_proving_sdk::{config::Config, prover::ProverBuilder, utils::init_tracing};
use snarkify_scroll_proving_sdk::prover::SnarkifyProver;

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
struct Args {
    /// Path to the configuration file in JSON format.
    /// Regarding the JSON format, please refer to the README.md for the configuration file template in
    /// https://github.com/snarkify/snarkify-scroll-proving-sdk
    #[arg(long = "config", default_value = "config.json")]
    config_file: String,
    /// Unique UUID for the service in Snarkify platform.
    #[arg(long = "service-id")]
    service_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let args = Args::parse();
    let cfg: Config = Config::from_file(args.config_file)?;
    let cloud_prover = SnarkifyProver::new(
        cfg.prover
            .cloud
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Missing cloud prover configuration"))?,
        args.service_id,
    );
    let prover = ProverBuilder::new(cfg)
        .with_proving_service(Box::new(cloud_prover))
        .build()
        .await?;

    prover.run().await;

    Ok(())
}
