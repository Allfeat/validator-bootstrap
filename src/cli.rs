use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// version tag (e.g v1.1.0) or branch (e.g master, develop...)
    #[arg(short, long, default_value = "master")]
    pub node_version: String,

    /// OVH API credentials to fetch validators keys from okms secret service.
    #[arg(short, long)]
    pub ovh_app_key: String,

    /// OVH API credentials to fetch validators keys from okms secret service.
    #[arg(long)]
    pub ovh_app_secret: String,

    /// OVH API credentials to fetch validators keys from okms secret service.
    #[arg(long)]
    pub ovh_consumer_key: String,

    /// OVH OKMS identifier linked to the desired path of validator keys.
    #[arg(long)]
    pub ovh_okms_id: String,

    /// OVH secret path containing validator keys.
    #[arg(long)]
    pub ovh_secret_path: String,

    /// The path to the chains data, this is the same as the Allfeat node.
    #[arg(long)]
    pub base_path: Option<String>,

    /// Extra arguments to start the validator node with.
    #[arg(last = true)]
    pub extra_args: Vec<String>,
}
