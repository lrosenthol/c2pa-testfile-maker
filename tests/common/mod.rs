use anyhow::Result;
use c2pa::{Builder, CallbackSigner, Ingredient, Reader, Relationship, SigningAlg};
use std::fs;
use std::path::{Path, PathBuf};

/// Test helper to get the path to test fixtures
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Test helper to get the path to test certificates
pub fn certs_dir() -> PathBuf {
    fixtures_dir().join("certs")
}

/// Test helper to get the path to test images
pub fn testfiles_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testfiles")
}

/// Test helper to get the path to manifest examples
pub fn manifests_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

/// Test helper to create output directory for test artifacts
pub fn output_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test_output");
    fs::create_dir_all(&dir).expect("Failed to create test output directory");
    dir
}

/// Helper function to sign a file with a manifest
pub fn sign_file_with_manifest(
    input_path: &Path,
    output_path: &Path,
    manifest_path: &Path,
) -> Result<()> {
    // Remove output file if it already exists
    if output_path.exists() {
        fs::remove_file(output_path)?;
    }

    // Read the manifest JSON
    let manifest_json = fs::read_to_string(manifest_path)?;

    // Create the builder from JSON
    let mut builder = Builder::from_json(&manifest_json)?;

    // Use the same test signer approach as c2pa-rs tests
    let signer = test_signer();

    // Sign and embed
    builder.sign_file(&signer, input_path, output_path)?;

    Ok(())
}

/// Helper function to sign a file with a manifest that includes file-based ingredients
/// This processes ingredients with file_path fields
pub fn sign_file_with_manifest_and_ingredients(
    input_path: &Path,
    output_path: &Path,
    manifest_path: &Path,
    ingredients_base_dir: &Path,
) -> Result<()> {
    // Remove output file if it already exists
    if output_path.exists() {
        fs::remove_file(output_path)?;
    }

    // Read the manifest JSON
    let manifest_json = fs::read_to_string(manifest_path)?;

    // Create the builder from JSON
    let mut builder = Builder::from_json(&manifest_json)?;

    // Process ingredients with file paths
    process_ingredients(&mut builder, &manifest_json, ingredients_base_dir)?;

    // Use the same test signer approach as c2pa-rs tests
    let signer = test_signer();

    // Sign and embed
    builder.sign_file(&signer, input_path, output_path)?;

    Ok(())
}

/// Process ingredients from manifest JSON and add them to the builder
fn process_ingredients(
    builder: &mut Builder,
    manifest_json: &str,
    ingredients_base_dir: &Path,
) -> Result<()> {
    use serde_json::Value as JsonValue;

    // Parse the manifest JSON to check for ingredients with file paths
    let manifest: JsonValue = serde_json::from_str(manifest_json)?;

    // Look for "ingredients_from_files" field
    if let Some(ingredients) = manifest
        .get("ingredients_from_files")
        .and_then(|v| v.as_array())
    {
        for ingredient_def in ingredients {
            // All entries in ingredients_from_files must have a file_path
            let file_path_str = ingredient_def
                .get("file_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing file_path in ingredient"))?;

            // Resolve the file path relative to the base directory
            let file_path = if Path::new(file_path_str).is_absolute() {
                PathBuf::from(file_path_str)
            } else {
                ingredients_base_dir.join(file_path_str)
            };

            if !file_path.exists() {
                anyhow::bail!("Ingredient file not found: {:?}", file_path);
            }

            // Load the ingredient file
            let mut source = fs::File::open(&file_path)?;

            // Determine format from file extension
            let extension = file_path
                .extension()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Ingredient file has no extension"))?;

            let format = extension_to_mime(extension)
                .ok_or_else(|| anyhow::anyhow!("Unsupported ingredient file format"))?;

            // Create an Ingredient from the file
            let mut ingredient = Ingredient::from_stream(format, &mut source)?;

            // Set the title if provided in the manifest
            if let Some(title) = ingredient_def.get("title").and_then(|v| v.as_str()) {
                ingredient.set_title(title);
            }

            // Set the relationship if provided
            if let Some(rel) = ingredient_def.get("relationship").and_then(|v| v.as_str()) {
                let relationship = match rel.to_lowercase().as_str() {
                    "parentof" => Relationship::ParentOf,
                    "componentof" => Relationship::ComponentOf,
                    _ => anyhow::bail!("Invalid relationship type: {}", rel),
                };
                ingredient.set_relationship(relationship);
            }

            // Add the ingredient to the builder
            builder.add_ingredient(ingredient);
        }
    }

    Ok(())
}

/// Converts a file extension to a MIME type
fn extension_to_mime(extension: &str) -> Option<&'static str> {
    Some(match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "tiff" | "tif" => "image/tiff",
        "bmp" => "image/bmp",
        _ => return None,
    })
}

/// Create a test signer using Ed25519 (same as c2pa-rs test infrastructure)
/// This uses the Ed25519 certificates from c2pa-rs which pass all validation
fn test_signer() -> CallbackSigner {
    const CERTS: &[u8] = include_bytes!("../fixtures/certs/ed25519.pub");
    const PRIVATE_KEY: &[u8] = include_bytes!("../fixtures/certs/ed25519.pem");

    let ed_signer = |_context: *const (), data: &[u8]| ed_sign(data, PRIVATE_KEY);
    CallbackSigner::new(ed_signer, SigningAlg::Ed25519, CERTS)
        .set_context("test" as *const _ as *const ())
}

fn ed_sign(data: &[u8], private_key: &[u8]) -> c2pa::Result<Vec<u8>> {
    use c2pa::crypto::raw_signature::RawSignerError;
    use ed25519_dalek::{Signature, Signer, SigningKey};
    use pem::parse;

    // Parse the PEM data to get the private key
    let pem = parse(private_key).map_err(|e| c2pa::Error::OtherError(Box::new(e)))?;

    // For Ed25519, the key is 32 bytes long, so we skip the first 16 bytes of the PEM data
    let key_bytes = &pem.contents()[16..];
    let signing_key = SigningKey::try_from(key_bytes)
        .map_err(|e| RawSignerError::InternalError(e.to_string()))?;

    // Sign the data
    let signature: Signature = signing_key.sign(data);
    Ok(signature.to_bytes().to_vec())
}

/// Helper function to verify a signed file has a valid manifest
pub fn verify_signed_file(file_path: &Path) -> Result<Reader> {
    let reader = Reader::from_file(file_path)?;

    // Ensure we have an active manifest
    assert!(
        reader.active_label().is_some(),
        "File should have an active manifest"
    );

    Ok(reader)
}

/// Helper to get all test image files
pub fn get_test_images() -> Vec<PathBuf> {
    let testfiles = testfiles_dir();
    vec![
        testfiles.join("Dog.jpg"),
        testfiles.join("Dog.png"),
        testfiles.join("Dog.webp"),
    ]
}

/// Helper function to extract manifest from a signed file
pub fn extract_manifest_to_file(input_path: &Path, output_path: &Path) -> Result<()> {
    // Remove output file if it already exists
    if output_path.exists() {
        fs::remove_file(output_path)?;
    }

    // Create a Reader from the input file
    let reader = Reader::from_file(input_path)?;

    // Ensure there's an active manifest
    let _active_label = reader
        .active_label()
        .ok_or_else(|| anyhow::anyhow!("No active C2PA manifest found"))?;

    // Get the manifest JSON
    let manifest_json = reader.json();

    // Create output directory if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Parse and re-serialize the JSON for pretty formatting
    let json_value: serde_json::Value = serde_json::from_str(&manifest_json)?;
    let pretty_json = serde_json::to_string_pretty(&json_value)?;

    fs::write(output_path, pretty_json)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixtures_exist() {
        assert!(testfiles_dir().exists(), "testfiles directory should exist");
        assert!(manifests_dir().exists(), "examples directory should exist");
        assert!(certs_dir().exists(), "test certs directory should exist");
    }

    #[test]
    fn test_images_exist() {
        for img in get_test_images() {
            assert!(img.exists(), "Test image should exist: {:?}", img);
        }
    }

    #[test]
    fn test_manifests_exist() {
        let simple = manifests_dir().join("simple_manifest.json");
        let full = manifests_dir().join("full_manifest.json");
        let asset_ref = manifests_dir().join("asset_ref_manifest.json");
        let asset_type = manifests_dir().join("asset_type_manifest.json");
        let cloud_data = manifests_dir().join("cloud_data_manifest.json");
        let depthmap_gdepth = manifests_dir().join("depthmap_gdepth_manifest.json");
        let external_reference = manifests_dir().join("external_reference_manifest.json");
        let actions_v2_edited = manifests_dir().join("actions_v2_edited_manifest.json");
        let actions_v2_translated = manifests_dir().join("actions_v2_translated_manifest.json");
        let actions_v2_redacted = manifests_dir().join("actions_v2_redacted_manifest.json");
        let actions_v2_cropped = manifests_dir().join("actions_v2_cropped_manifest.json");
        let actions_v2_filtered = manifests_dir().join("actions_v2_filtered_manifest.json");

        assert!(simple.exists(), "simple_manifest.json should exist");
        assert!(full.exists(), "full_manifest.json should exist");
        assert!(asset_ref.exists(), "asset_ref_manifest.json should exist");
        assert!(asset_type.exists(), "asset_type_manifest.json should exist");
        assert!(cloud_data.exists(), "cloud_data_manifest.json should exist");
        assert!(
            depthmap_gdepth.exists(),
            "depthmap_gdepth_manifest.json should exist"
        );
        assert!(
            external_reference.exists(),
            "external_reference_manifest.json should exist"
        );
        assert!(
            actions_v2_edited.exists(),
            "actions_v2_edited_manifest.json should exist"
        );
        assert!(
            actions_v2_translated.exists(),
            "actions_v2_translated_manifest.json should exist"
        );
        assert!(
            actions_v2_redacted.exists(),
            "actions_v2_redacted_manifest.json should exist"
        );
        assert!(
            actions_v2_cropped.exists(),
            "actions_v2_cropped_manifest.json should exist"
        );
        assert!(
            actions_v2_filtered.exists(),
            "actions_v2_filtered_manifest.json should exist"
        );
    }

    #[test]
    fn test_certs_exist() {
        let cert = certs_dir().join("es256_cert.pem");
        let key = certs_dir().join("es256_private.pem");

        assert!(cert.exists(), "Test certificate should exist");
        assert!(key.exists(), "Test private key should exist");
    }
}
