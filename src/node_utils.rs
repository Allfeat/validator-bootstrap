use std::{path::Path, process::Command};

use anyhow::Context;

use crate::SecretData;

pub fn inject_session_keys(
    node_path: &Path,
    chain_spec: &Path,
    secrets: &SecretData,
    base_path: &[&str],
) -> anyhow::Result<()> {
    // Inject Grandpa key
    Command::new(node_path)
        .args([
            "key",
            "insert",
            "--chain",
            chain_spec
                .to_str()
                .context("invalid chain spec path utf-8")?,
            "--scheme",
            "Ed25519",
            "--suri",
            &secrets.grandpa,
            "--key-type",
            "gran",
            "--password",
            &secrets.keystore_secret,
        ])
        .args(base_path)
        .status()?;
    // Inject Aura key
    Command::new(node_path)
        .args([
            "key",
            "insert",
            "--chain",
            chain_spec
                .to_str()
                .context("invalid chain spec path utf-8")?,
            "--scheme",
            "Sr25519",
            "--suri",
            &secrets.aura,
            "--key-type",
            "aura",
            "--password",
            &secrets.keystore_secret,
        ])
        .args(base_path)
        .status()?;
    // Inject ImOnline key
    Command::new(node_path)
        .args([
            "key",
            "insert",
            "--chain",
            chain_spec
                .to_str()
                .context("invalid chain spec path utf-8")?,
            "--scheme",
            "Sr25519",
            "--suri",
            &secrets.im_online,
            "--key-type",
            "imon",
            "--password",
            &secrets.keystore_secret,
        ])
        .args(base_path)
        .status()?;

    println!("Finished injecting session keys.");
    Ok(())
}
