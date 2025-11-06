use std::process::Command;

use clap::Parser;
use reqwest::blocking::Response;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Method, blocking::Client};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use urlencoding::encode;

use crate::cli::Cli;
use crate::downloader::{get_chain_spec, get_node_binary};
use crate::node_utils::inject_session_keys;

mod cli;
mod downloader;
mod node_utils;

#[derive(Debug, Clone, Default)]
struct OvhClient {
    app_key: String,
    app_secret: String,
    consumer_key: String,
    api_base_v2: String, // ex: "https://eu.api.ovh.com/v2"
    http: Client,
}

#[derive(Deserialize)]
struct SecretData {
    aura: String,
    grandpa: String,
    im_online: String,

    keystore_secret: String,
    node_key: String,
}
#[derive(Deserialize)]
struct SecretResponse {
    version: Version,
}
#[derive(Deserialize)]
struct Version {
    data: SecretData,
}

impl OvhClient {
    pub fn new(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        consumer_key: impl Into<String>,
        api_base_v2: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let http = Client::builder().build()?;
        Ok(Self {
            app_key: app_key.into(),
            app_secret: app_secret.into(),
            consumer_key: consumer_key.into(),
            api_base_v2: api_base_v2.into(),
            http,
        })
    }

    pub fn ovh_time(&self) -> anyhow::Result<u64> {
        let t = reqwest::blocking::get("https://api.ovh.com/1.0/auth/time")?.text()?;
        Ok(t.trim().parse::<u64>()?)
    }

    fn sign(&self, method: &Method, url: &str, body: &str, ts: u64) -> String {
        let method_str = method.as_str();
        let clear = format!(
            "{}+{}+{}+{}+{}+{}",
            self.app_secret, self.consumer_key, method_str, url, body, ts
        );
        let mut hasher = Sha1::new();
        hasher.update(clear.as_bytes());
        format!("$1${:x}", hasher.finalize())
    }

    pub fn send(
        &self,
        method: Method,
        url: &str,
        body: &str,
    ) -> anyhow::Result<reqwest::blocking::Response> {
        let ts = self.ovh_time()?;
        let sig = self.sign(&method, url, body, ts);

        let mut headers = HeaderMap::new();
        if !body.is_empty() {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json;charset=utf-8"),
            );
        }
        headers.insert("X-Ovh-Application", HeaderValue::from_str(&self.app_key)?);
        headers.insert("X-Ovh-Timestamp", HeaderValue::from_str(&ts.to_string())?);
        headers.insert("X-Ovh-Signature", HeaderValue::from_str(&sig)?);
        headers.insert("X-Ovh-Consumer", HeaderValue::from_str(&self.consumer_key)?);

        let req = self.http.request(method, url).headers(headers);
        let req = if body.is_empty() {
            req
        } else {
            req.body(body.to_string())
        };

        Ok(req.send()?)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let node_bin = get_node_binary(&cli.node_version)?;
    let spec_path = get_chain_spec()?;

    let client = OvhClient::new(
        cli.ovh_app_key,
        cli.ovh_app_secret,
        cli.ovh_consumer_key,
        "https://eu.api.ovh.com/v2",
    )?;
    let okms_id = cli.ovh_okms_id;
    let path = cli.ovh_secret_path;
    let response = get_secret_for(&client, &okms_id, &path)?;
    let secrets_resp: SecretResponse = response.json()?;

    let base_path_arg: &[&str] = match cli.base_path {
        Some(base_path) => &["--base-path", &base_path.clone()],
        None => &[],
    };

    inject_session_keys(
        &node_bin,
        &spec_path,
        &secrets_resp.version.data,
        base_path_arg,
    )?;

    let status = Command::new(node_bin)
        .args(base_path_arg)
        .arg("--validator")
        .arg("--node-key")
        .arg(secrets_resp.version.data.node_key)
        .args(["--chain", spec_path.to_str().unwrap_or_default()])
        .args(["--password", &secrets_resp.version.data.keystore_secret])
        .args(&cli.extra_args)
        .status()?;
    std::process::exit(status.code().unwrap_or(1));
}

fn get_secret_for(client: &OvhClient, okms_id: &str, path: &str) -> anyhow::Result<Response> {
    let path_dynamic = encode(path);

    let url = format!(
        "{}/okms/resource/{}/secret/{}?includeData=true",
        client.api_base_v2, okms_id, path_dynamic
    );

    let resp = client.send(Method::GET, &url, "")?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use crate::OvhClient;

    #[test]
    fn ovh_time_return_valid_time() {
        let c = OvhClient::default();

        assert!(c.ovh_time().unwrap() > 0u64)
    }
}
