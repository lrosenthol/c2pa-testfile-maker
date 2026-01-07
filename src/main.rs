use anyhow::{Context, Result};
use c2pa::{create_signer, Builder, SigningAlg};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// C2PA Testfile Maker - Create and embed C2PA manifests into media assets
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the JSON manifest configuration file
    #[arg(short, long, value_name = "FILE")]
    manifest: PathBuf,

    /// Path to the input media asset (JPEG, PNG, etc.)
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Path to the output file or directory
    #[arg(short, long, value_name = "PATH")]
    output: PathBuf,

    /// Path to the certificate file (PEM format)
    #[arg(short, long, value_name = "FILE")]
    cert: PathBuf,

    /// Path to the private key file (PEM format)
    #[arg(short, long, value_name = "FILE")]
    key: PathBuf,

    /// Signing algorithm (es256, es384, es512, ps256, ps384, ps512, ed25519)
    #[arg(short, long, default_value = "es256")]
    algorithm: String,
}

fn determine_output_path(input: &Path, output: &Path) -> Result<PathBuf> {
    if output.is_dir() {
        let filename = input.file_name().context("Input file has no filename")?;
        Ok(output.join(filename))
    } else {
        Ok(output.to_path_buf())
    }
}

fn parse_signing_algorithm(alg: &str) -> Result<SigningAlg> {
    match alg.to_lowercase().as_str() {
        "es256" => Ok(SigningAlg::Es256),
        "es384" => Ok(SigningAlg::Es384),
        "es512" => Ok(SigningAlg::Es512),
        "ps256" => Ok(SigningAlg::Ps256),
        "ps384" => Ok(SigningAlg::Ps384),
        "ps512" => Ok(SigningAlg::Ps512),
        "ed25519" => Ok(SigningAlg::Ed25519),
        _ => anyhow::bail!("Unsupported signing algorithm: {}", alg),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read and parse the JSON manifest configuration
    let manifest_json =
        fs::read_to_string(&cli.manifest).context("Failed to read manifest JSON file")?;

    // Validate input file exists
    if !cli.input.exists() {
        anyhow::bail!("Input file does not exist: {:?}", cli.input);
    }

    // Determine the output path
    let output_path = determine_output_path(&cli.input, &cli.output)?;

    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).context("Failed to create output directory")?;
    }

    // Parse signing algorithm
    let signing_alg = parse_signing_algorithm(&cli.algorithm)?;

    println!("Creating C2PA manifest...");
    println!("  Input: {:?}", cli.input);
    println!("  Output: {:?}", output_path);
    println!("  Algorithm: {}", cli.algorithm);

    // Create a builder from the JSON manifest
    let mut builder = Builder::from_json(&manifest_json)
        .context("Failed to create builder from JSON manifest")?;

    // Create a signer from the certificate and private key files
    let signer = create_signer::from_files(
        cli.cert.to_str().context("Invalid cert path")?,
        cli.key.to_str().context("Invalid key path")?,
        signing_alg,
        None,
    )
    .context("Failed to create signer")?;

    // Sign and embed the manifest into the asset
    builder
        .sign_file(&*signer, &cli.input, &output_path)
        .context("Failed to sign and embed manifest")?;

    println!("✓ Successfully created and embedded C2PA manifest");
    println!("  Output file: {:?}", output_path);

    Ok(())
}
