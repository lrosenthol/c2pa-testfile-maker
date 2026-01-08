use anyhow::Result;
use std::path::{Path, PathBuf};

mod common;

use common::{
    get_test_images, manifests_dir, output_dir, sign_file_with_manifest, verify_signed_file,
};

/// Generate output filename from input filename and manifest type
fn generate_output_name(input: &Path, manifest_type: &str) -> PathBuf {
    let stem = input.file_stem().unwrap().to_str().unwrap();
    let ext = input.extension().unwrap().to_str().unwrap();
    output_dir().join(format!("{}_{}.{}", stem, manifest_type, ext))
}

// Tests for Dog.jpg
#[test]
fn test_dog_jpg_simple_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.jpg");
    let manifest = manifests_dir().join("simple_manifest.json");
    let output = generate_output_name(&input, "simple");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify basic manifest properties
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Example Image with C2PA Manifest"
        );
    }

    println!("✓ Dog.jpg with simple_manifest.json: {}", output.display());
    Ok(())
}

#[test]
fn test_dog_jpg_full_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.jpg");
    let manifest = manifests_dir().join("full_manifest.json");
    let output = generate_output_name(&input, "full");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify detailed manifest properties
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Edited Photo with Complete Metadata"
        );

        // Verify we have assertions
        assert!(!manifest.assertions().is_empty());
    }

    println!("✓ Dog.jpg with full_manifest.json: {}", output.display());
    Ok(())
}

// Tests for Dog.png
#[test]
fn test_dog_png_simple_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.png");
    let manifest = manifests_dir().join("simple_manifest.json");
    let output = generate_output_name(&input, "simple");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify basic manifest properties
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Example Image with C2PA Manifest"
        );
    }

    println!("✓ Dog.png with simple_manifest.json: {}", output.display());
    Ok(())
}

#[test]
fn test_dog_png_full_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.png");
    let manifest = manifests_dir().join("full_manifest.json");
    let output = generate_output_name(&input, "full");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify detailed manifest properties
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Edited Photo with Complete Metadata"
        );

        // Verify we have assertions
        assert!(!manifest.assertions().is_empty());
    }

    println!("✓ Dog.png with full_manifest.json: {}", output.display());
    Ok(())
}

// Tests for Dog.webp
#[test]
fn test_dog_webp_simple_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.webp");
    let manifest = manifests_dir().join("simple_manifest.json");
    let output = generate_output_name(&input, "simple");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify basic manifest properties
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Example Image with C2PA Manifest"
        );
    }

    println!("✓ Dog.webp with simple_manifest.json: {}", output.display());
    Ok(())
}

#[test]
fn test_dog_webp_full_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.webp");
    let manifest = manifests_dir().join("full_manifest.json");
    let output = generate_output_name(&input, "full");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify detailed manifest properties
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Edited Photo with Complete Metadata"
        );

        // Verify we have assertions
        assert!(!manifest.assertions().is_empty());
    }

    println!("✓ Dog.webp with full_manifest.json: {}", output.display());
    Ok(())
}

// Batch test to run all combinations
#[test]
fn test_all_images_all_manifests() -> Result<()> {
    let manifests = vec![
        ("simple", manifests_dir().join("simple_manifest.json")),
        ("full", manifests_dir().join("full_manifest.json")),
        ("asset_ref", manifests_dir().join("asset_ref_manifest.json")),
        ("asset_type", manifests_dir().join("asset_type_manifest.json")),
    ];

    let mut success_count = 0;
    let mut total_count = 0;

    for input in get_test_images() {
        for (manifest_type, manifest_path) in &manifests {
            total_count += 1;
            let output = generate_output_name(&input, manifest_type);

            match sign_file_with_manifest(&input, &output, manifest_path) {
                Ok(_) => match verify_signed_file(&output) {
                    Ok(_) => {
                        success_count += 1;
                        println!(
                            "✓ {} with {} manifest",
                            input.file_name().unwrap().to_str().unwrap(),
                            manifest_type
                        );
                    }
                    Err(e) => {
                        eprintln!("✗ Verification failed for {:?}: {}", output, e);
                    }
                },
                Err(e) => {
                    eprintln!(
                        "✗ Signing failed for {:?} with {}: {}",
                        input, manifest_type, e
                    );
                }
            }
        }
    }

    println!("\n{}/{} tests passed", success_count, total_count);
    assert_eq!(
        success_count, total_count,
        "All image/manifest combinations should succeed"
    );

    Ok(())
}

// Tests for asset-type manifest
#[test]
fn test_dog_jpg_asset_type_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.jpg");
    let manifest = manifests_dir().join("asset_type_manifest.json");
    let output = generate_output_name(&input, "asset_type");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify manifest properties and asset-type assertion
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Image with Asset Type Information"
        );

        // Verify we have assertions including asset-type
        let assertions = manifest.assertions();
        assert!(!assertions.is_empty());

        // Check that c2pa.asset-type assertion exists
        let has_asset_type = assertions.iter().any(|a| a.label() == "c2pa.asset-type");
        assert!(has_asset_type, "Should have c2pa.asset-type assertion");
    }

    println!("✓ Dog.jpg with asset_type_manifest.json: {}", output.display());
    Ok(())
}

#[test]
fn test_dog_png_asset_type_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.png");
    let manifest = manifests_dir().join("asset_type_manifest.json");
    let output = generate_output_name(&input, "asset_type");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify manifest properties and asset-type assertion
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Image with Asset Type Information"
        );

        // Verify we have the asset-type assertion
        let assertions = manifest.assertions();
        let has_asset_type = assertions.iter().any(|a| a.label() == "c2pa.asset-type");
        assert!(has_asset_type, "Should have c2pa.asset-type assertion");
    }

    println!("✓ Dog.png with asset_type_manifest.json: {}", output.display());
    Ok(())
}

#[test]
fn test_dog_webp_asset_type_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.webp");
    let manifest = manifests_dir().join("asset_type_manifest.json");
    let output = generate_output_name(&input, "asset_type");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify manifest properties and asset-type assertion
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Image with Asset Type Information"
        );

        // Verify we have the asset-type assertion
        let assertions = manifest.assertions();
        let has_asset_type = assertions.iter().any(|a| a.label() == "c2pa.asset-type");
        assert!(has_asset_type, "Should have c2pa.asset-type assertion");
    }

    println!("✓ Dog.webp with asset_type_manifest.json: {}", output.display());
    Ok(())
}

// Tests for asset-ref manifest
#[test]
fn test_dog_jpg_asset_ref_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.jpg");
    let manifest = manifests_dir().join("asset_ref_manifest.json");
    let output = generate_output_name(&input, "asset_ref");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify manifest properties and asset-ref assertion
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Image with Asset Reference"
        );

        // Verify we have assertions including asset-ref
        let assertions = manifest.assertions();
        assert!(!assertions.is_empty());

        // Check that c2pa.asset-ref assertion exists
        let has_asset_ref = assertions.iter().any(|a| a.label() == "c2pa.asset-ref");
        assert!(has_asset_ref, "Should have c2pa.asset-ref assertion");
    }

    println!("✓ Dog.jpg with asset_ref_manifest.json: {}", output.display());
    Ok(())
}

#[test]
fn test_dog_png_asset_ref_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.png");
    let manifest = manifests_dir().join("asset_ref_manifest.json");
    let output = generate_output_name(&input, "asset_ref");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify manifest properties and asset-ref assertion
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Image with Asset Reference"
        );

        // Verify we have the asset-ref assertion
        let assertions = manifest.assertions();
        let has_asset_ref = assertions.iter().any(|a| a.label() == "c2pa.asset-ref");
        assert!(has_asset_ref, "Should have c2pa.asset-ref assertion");
    }

    println!("✓ Dog.png with asset_ref_manifest.json: {}", output.display());
    Ok(())
}

#[test]
fn test_dog_webp_asset_ref_manifest() -> Result<()> {
    let input = common::testfiles_dir().join("Dog.webp");
    let manifest = manifests_dir().join("asset_ref_manifest.json");
    let output = generate_output_name(&input, "asset_ref");

    sign_file_with_manifest(&input, &output, &manifest)?;

    let reader = verify_signed_file(&output)?;
    assert!(reader.active_label().is_some());

    // Verify manifest properties and asset-ref assertion
    if let Some(manifest_label) = reader.active_label() {
        let manifest = reader.get_manifest(manifest_label).unwrap();
        assert_eq!(
            manifest.title().unwrap_or_default(),
            "Image with Asset Reference"
        );

        // Verify we have the asset-ref assertion
        let assertions = manifest.assertions();
        let has_asset_ref = assertions.iter().any(|a| a.label() == "c2pa.asset-ref");
        assert!(has_asset_ref, "Should have c2pa.asset-ref assertion");
    }

    println!("✓ Dog.webp with asset_ref_manifest.json: {}", output.display());
    Ok(())
}

// Test to verify output files are valid and readable
#[test]
fn test_output_files_are_readable() {
    let output = output_dir();
    if output.exists() {
        for entry in std::fs::read_dir(output).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let metadata = std::fs::metadata(&path).unwrap();
                assert!(
                    metadata.len() > 0,
                    "Output file should not be empty: {:?}",
                    path
                );
            }
        }
    }
}
