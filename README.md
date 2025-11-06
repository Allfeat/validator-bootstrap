# Allfeat Validator Bootstrapping

A Rust-based command-line wrapper that securely retrieves validator session keys and credentials from **OVH Cloud OKMS**, downloads the latest **Allfeat node binary** and **chain specification**, injects keys into the node keystore, and launches the validator process.

This tool simplifies validator deployment and key management in production environments.

---

## ✨ Features

- 🔐 **Secure key injection** — Retrieves session keys (`aura`, `grandpa`, `imonline`, etc.) from OVH OKMS and injects them safely into the node keystore.
- ⚙️ **Automatic node binary management** — Downloads the correct Allfeat node version from the OVH S3 release bucket.
- 🤶 **Automatic chain specification** — Fetches the latest chain spec (`melodie_raw.json`) from the Allfeat GitHub repository.
- 🥉 **Custom runtime arguments** — Allows passing extra arguments directly to the validator node (`--bootnodes`, `--rpc-external`, etc.).
- 🧱 **Built-in integrity and permission handling** — Applies proper executable permissions and basic validation on all files.
- 🤰 **Integration-ready** — Suitable for automation pipelines or deployment scripts on Linux-based infrastructure.

---

## 🧠 Overview

This tool performs the following sequence:

1. **Fetch secrets** from OVH OKMS (`aura`, `grandpa`, `imonline`, `node_key`, `keystore_secret`).
2. **Download** the Allfeat node binary from the S3 release endpoint.
3. **Download** the chain spec (`melodie_raw.json`) from GitHub.
4. **Inject session keys** into the keystore.
5. **Launch** the validator node with the provided configuration and extra arguments.

---

## 🏷️ Installation

### Prerequisites

- Rust (1.74+ recommended)
- Access to an OVH OKMS instance containing your validator secrets
- Network access to:
  - `https://eu.api.ovh.com/v2`
  - Allfeat GitHub repository
  - Allfeat binary release S3 bucket

### Build

```bash
git clone https://github.com/Allfeat/Allfeat-validator-launcher.git
cd Allfeat-validator-launcher
cargo build --release
```

The binary will be available at:

```
./target/release/allfeat-validator-launcher
```

---

## 🚀 Usage

### Basic Example

```bash
./allfeat-validator-launcher \
  --node-version master \
  --ovh-app-key $OVH_APP_KEY \
  --ovh-app-secret $OVH_APP_SECRET \
  --ovh-consumer-key $OVH_CONSUMER_KEY \
  --ovh-okms-id $OKMS_ID \
  --ovh-secret-path /validator/secrets \
  -- \
  --bootnodes /ip4/192.168.1.2/tcp/30333/p2p/QmPeerId \
  --rpc-external \
  --prometheus-port 9615
```

### Options

| Flag                   | Description                                                                     |
| ---------------------- | ------------------------------------------------------------------------------- |
| `--node-version <ver>` | Target Allfeat node version (e.g. `master`, `v0.9.1`)                           |
| `--ovh-app-key`        | OVH API application key                                                         |
| `--ovh-app-secret`     | OVH API application secret                                                      |
| `--ovh-consumer-key`   | OVH consumer key (bound to your OKMS permissions)                               |
| `--ovh-okms-id`        | OKMS resource identifier                                                        |
| `--ovh-secret-path`    | Path of the secret inside OKMS (e.g. `/validator1`)                             |
| `--`                   | Separator; everything after this is passed directly to the Allfeat node process |

---

## 🤩 OVH Integration

The tool authenticates and signs each OVH request following the OVHv2 API spec:

```
X-Ovh-Application: <app_key>
X-Ovh-Consumer: <consumer_key>
X-Ovh-Timestamp: <server_time>
X-Ovh-Signature: $1$<sha1(app_secret + consumer_key + method + url + body + timestamp)>
```

The retrieved secret payload must include:

```json
{
  "aura": "<mnemonic or secret URI>",
  "grandpa": "<mnemonic or secret URI>",
  "im_online": "<mnemonic or secret URI>",
  "node_key": "<hex or base58 key>",
  "keystore_secret": "<password>"
}
```

---

## 🧰 Development

Run locally (non-production) with test data:

```bash
cargo run -- \
  --node-version master \
  --ovh-app-key dummy \
  --ovh-app-secret dummy \
  --ovh-consumer-key dummy \
  --ovh-okms-id dummy \
  --ovh-secret-path dummy
```

---

## 🧪 Tests

Some tests require network access; they are marked as `#[ignore]` to avoid CI failures.

```bash
cargo test -- --ignored
```

---

## ⚠️ Security Notes

- **Do not log or print secrets** (`node_key`, `keystore_secret`, or mnemonic URIs).
- **Do not store secrets in plain text** — only in secure backends like OVH OKMS.
- **Prefer using `--password-filename` or stdin** over `--password` in CLI arguments.
- Keep the **keystore directory private (chmod 700)**.
- Always validate downloaded binaries using **checksums** before execution.

---

## 📦 File Structure

```
src/
├── cli.rs             # Clap CLI definition
├── downloader.rs      # Handles binary and chain spec downloads
├── node_utils.rs      # Key injection into node keystore
├── main.rs            # Main orchestration logic
```

---

## 🏁 License

MIT © Allfeat Foundation
Developed by the **Allfeat Foundation Core Team**

---

## 📞 Support

- Website: [https://allfeat.org](https://allfeat.org)
- Documentation: [https://docs.allfeat.org](https://docs.allfeat.org)
