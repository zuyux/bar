use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

const PROTOCOL_ID: &str = "brc-app";
const REGISTRY_ADDRESS: &str = "bc1p0saw6z028y7h6eag3w6hx5an6mk5ta8qk7wx2d3gtqtrty243uvqvjzvew";
const GENESIS_TIMESTAMP: u64 = 1_745_092_800;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Path to the ord binary.
    #[arg(long, default_value = "ord", global = true)]
    ord_bin: String,

    /// Bitcoin network passed to ord.
    #[arg(long, value_enum, default_value_t = Network::Mainnet, global = true)]
    network: Network,

    /// Output directory for generated inscription JSON files.
    #[arg(long, default_value = "inscriptions/generated", global = true)]
    out_dir: PathBuf,

    /// Print the ord command but do not run it.
    #[arg(long, global = true)]
    print_only: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create and inscribe the canonical BAR genesis payload.
    Genesis(InscribeArgs),

    /// Create and inscribe the first BBOX app registration payload.
    Bbox(BboxArgs),
}

#[derive(Parser, Debug)]
struct InscribeArgs {
    /// Fee rate in sats/vB.
    #[arg(long)]
    fee_rate: f64,

    /// Destination address for the inscription.
    #[arg(long, default_value = REGISTRY_ADDRESS)]
    destination: String,

    /// Optional ord postage amount, for example 330sats.
    #[arg(long)]
    postage: Option<String>,

    /// Broadcast the inscription. Without this flag, ord is run with --dry-run.
    #[arg(long)]
    execute: bool,
}

#[derive(Parser, Debug)]
struct BboxArgs {
    #[command(flatten)]
    inscribe: InscribeArgs,

    /// Current publisher Taproot address for BBOX.
    #[arg(long)]
    owner: String,

    /// BBOX source repository URL.
    #[arg(long)]
    repo: String,

    /// BBOX license identifier.
    #[arg(long, default_value = "MIT")]
    license: String,

    /// First published BBOX version.
    #[arg(long, default_value = "0.1.0")]
    version: String,

    /// Build hash, preferably sha256:<hex>.
    #[arg(long)]
    build_hash: String,

    /// Supported platform. Repeat for multiple values.
    #[arg(long = "platform", default_value = "web")]
    platforms: Vec<String>,

    /// BAR chain_layer value.
    #[arg(long, default_value = "BTC")]
    chain_layer: String,

    /// Unix timestamp for the app registration. Defaults to current UTC time.
    #[arg(long)]
    timestamp: Option<u64>,

    /// App description to place in the first BBOX registration.
    #[arg(
        long,
        default_value = "BBOX integration for browsing and indexing BAR/BRC-App inscriptions."
    )]
    description: String,
}

#[derive(Clone, Debug, ValueEnum)]
enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

impl Network {
    fn ord_flag(&self) -> Option<&'static str> {
        match self {
            Self::Mainnet => None,
            Self::Testnet => Some("--testnet"),
            Self::Signet => Some("--signet"),
            Self::Regtest => Some("--regtest"),
        }
    }
}

#[derive(Serialize)]
struct GenesisInscription<'a> {
    p: &'a str,
    op: &'a str,
    name: &'a str,
    description: &'a str,
    creator: &'a str,
    registry_address: &'a str,
    timestamp: u64,
}

#[derive(Serialize)]
struct RegisterInscription {
    p: &'static str,
    op: &'static str,
    app_id: &'static str,
    owner: String,
    name: &'static str,
    repo: String,
    description: String,
    license: String,
    version: String,
    build_hash: String,
    platform: Vec<String>,
    chain_layer: String,
    previous: Option<String>,
    timestamp: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Genesis(args) => {
            let payload = GenesisInscription {
                p: PROTOCOL_ID,
                op: "genesis",
                name: "Bitcoin App Registry (BAR)",
                description: "Permissionless, no-gatekeeper on-chain registry of open-source applications on Bitcoin L1. Publishers retain full control to update metadata and transfer ownership. Append-only immutable history. Built as a censorship-resistant alternative after events like the Bitchat China ban.",
                creator: "community-driven",
                registry_address: REGISTRY_ADDRESS,
                timestamp: GENESIS_TIMESTAMP,
            };

            let file = write_payload(&cli.out_dir, "bar-genesis.json", &payload)?;
            run_ord_inscribe(&cli, args, &file)?;
        }
        Commands::Bbox(args) => {
            validate_taproot_address(&args.owner, "owner")?;
            validate_taproot_address(&args.inscribe.destination, "destination")?;
            validate_build_hash(&args.build_hash)?;

            let payload = RegisterInscription {
                p: PROTOCOL_ID,
                op: "register",
                app_id: "bbox",
                owner: args.owner.clone(),
                name: "BBOX",
                repo: args.repo.clone(),
                description: args.description.clone(),
                license: args.license.clone(),
                version: args.version.clone(),
                build_hash: args.build_hash.clone(),
                platform: args.platforms.clone(),
                chain_layer: args.chain_layer.clone(),
                previous: None,
                timestamp: args.timestamp.unwrap_or_else(current_unix_timestamp),
            };

            let file = write_payload(&cli.out_dir, "bbox-register.json", &payload)?;
            run_ord_inscribe(&cli, &args.inscribe, &file)?;
        }
    }

    Ok(())
}

fn write_payload<T: Serialize>(out_dir: &Path, name: &str, payload: &T) -> Result<PathBuf> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;

    let path = out_dir.join(name);
    let json = serde_json::to_string_pretty(payload)?;
    fs::write(&path, format!("{json}\n"))
        .with_context(|| format!("failed to write {}", path.display()))?;

    println!("Wrote {}", path.display());
    println!("{json}");
    Ok(path)
}

fn run_ord_inscribe(cli: &Cli, args: &InscribeArgs, file: &Path) -> Result<()> {
    validate_taproot_address(&args.destination, "destination")?;

    let mut command = Command::new(&cli.ord_bin);
    if let Some(flag) = cli.network.ord_flag() {
        command.arg(flag);
    }

    command
        .arg("wallet")
        .arg("inscribe")
        .arg("--fee-rate")
        .arg(args.fee_rate.to_string());

    if let Some(postage) = &args.postage {
        command.arg("--postage").arg(postage);
    }

    command
        .arg("--file")
        .arg(file)
        .arg("--destination")
        .arg(&args.destination);

    if !args.execute {
        command.arg("--dry-run");
    }

    let rendered = render_command(&command);
    println!("Command: {rendered}");

    if cli.print_only {
        return Ok(());
    }

    let status = command
        .status()
        .with_context(|| format!("failed to run `{}`", cli.ord_bin))?;

    if !status.success() {
        bail!("ord exited with status {status}");
    }

    Ok(())
}

fn render_command(command: &Command) -> String {
    let mut parts = vec![command.get_program().to_string_lossy().to_string()];
    parts.extend(
        command
            .get_args()
            .map(|arg| shell_quote(&arg.to_string_lossy())),
    );
    parts.join(" ")
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "-_./:=@".contains(c))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

fn validate_taproot_address(address: &str, field: &str) -> Result<()> {
    if !address.starts_with("bc1p")
        && !address.starts_with("tb1p")
        && !address.starts_with("bcrt1p")
    {
        bail!("{field} must be a Taproot address starting with bc1p, tb1p, or bcrt1p");
    }

    Ok(())
}

fn validate_build_hash(build_hash: &str) -> Result<()> {
    if !build_hash.starts_with("sha256:") {
        bail!("build_hash should use the sha256:<hex> format");
    }

    let hex = &build_hash["sha256:".len()..];
    if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("build_hash must contain a 64-character sha256 hex digest");
    }

    Ok(())
}

fn current_unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock is before unix epoch")
        .as_secs()
}
