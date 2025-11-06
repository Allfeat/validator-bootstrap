use std::{fs, io::Write, os::unix::fs::PermissionsExt, path::PathBuf, time::Duration};

use reqwest::blocking::{Client, ClientBuilder};
use serde_json::Value;

const S3_NODE_ENDPOINT: &str = "allfeat-prod-binaries-releases.s3.eu-west-par.io.cloud.ovh.net";
const CHAIN_SPEC_URL: &str = "https://raw.githubusercontent.com/Allfeat/Allfeat/refs/heads/master/node/specs/testnets/melodie/v2/melodie_raw.json";

pub fn get_node_binary(version: &str) -> anyhow::Result<PathBuf> {
    println!("Downloading Allfeat node ({version}) ...");
    let dl_url = format!(
        "https://{S3_NODE_ENDPOINT}/{version}/allfeat-linux-{}",
        arch_suffix()
    );
    let bin_path: PathBuf = ["./allfeat"].iter().collect();

    let downloader = ClientBuilder::new()
        .timeout(Duration::from_secs(60))
        .build()?;
    let mut resp = downloader.get(dl_url).send()?.error_for_status()?;

    let mut file = fs::File::create(&bin_path)?;
    resp.copy_to(&mut file)?;
    file.flush()?;

    // Give permissions to exec
    let mut perms = fs::metadata(&bin_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bin_path, perms)?;

    println!("Node downloaded to: {}", bin_path.to_string_lossy());
    Ok(bin_path)
}

pub fn get_chain_spec() -> anyhow::Result<PathBuf> {
    let spec_path: PathBuf = ["./chain_spec.json"].iter().collect();

    let client = Client::new();
    let resp = client.get(CHAIN_SPEC_URL).send()?.error_for_status()?;

    let json: Value = resp.json()?;

    let mut file = fs::File::create(&spec_path)?;
    file.write_all(json.to_string().as_bytes())?;
    file.flush()?;

    Ok(spec_path)
}

fn arch_suffix() -> String {
    #[cfg(target_arch = "x86_64")]
    return String::from("x86_64");
    #[cfg(target_arch = "aarch64")]
    return String::from("aarch64");
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    compile_error!("Unsupported architecture");
}

#[cfg(test)]
mod tests {
    use crate::downloader::{get_chain_spec, get_node_binary};

    #[test]
    fn download_work() {
        get_node_binary("master").unwrap();
    }

    #[test]
    fn download_chain_spec_work() {
        get_chain_spec().unwrap();
    }
}
