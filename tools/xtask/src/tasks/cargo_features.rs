use anyhow::Result;
use cargo_metadata::Metadata;
use clap::Parser;
use toml_edit::{Array, DocumentMut, Item, Table, Value};

static DEFAULT_FEATURES: [&str; 0] = [];

#[derive(Parser)]
pub struct CargoFeaturesCommandArgs {}

pub async fn run_cargo_features(_args: CargoFeaturesCommandArgs, metadata: Metadata) -> Result<()> {
    for pkg_id in metadata.workspace_members {
        let package = metadata
            .packages
            .iter()
            .find(|p| p.id == pkg_id)
            .expect("package from workspace_members not found in packages");

        let manifest_path = &package.manifest_path;
        let content = tokio::fs::read_to_string(&manifest_path).await?;
        let mut doc = content.parse::<DocumentMut>()?;

        // If DEFAULT_FEATURES is empty, remove cargo-features section and any spacing after it
        if DEFAULT_FEATURES.is_empty() {
            if doc.get("cargo-features").is_some() {
                doc.as_table_mut().remove("cargo-features");
                tokio::fs::write(&manifest_path, doc.to_string()).await?;
                println!("Removed cargo-features from {}", manifest_path.to_path_buf());
            }
            continue;
        }

        // Collect existing cargo-features
        let mut existing: Vec<String> =
            if let Some(Item::Value(Value::Array(arr))) = doc.get("cargo-features") {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            } else {
                Vec::new()
            };

        // Merge defaults without duplicates
        for &feat in &DEFAULT_FEATURES {
            if !existing.iter().any(|e| e == feat) {
                existing.push(feat.to_string());
            }
        }

        // Build new TOML array
        let mut new_arr = Array::new();
        for feat in &existing {
            new_arr.push(feat.as_str());
        }

        // Prepare item and insert as first table entry and remove old section if present
        let item = Item::Value(Value::Array(new_arr));
        doc.as_table_mut().remove("cargo-features");

        // Rebuild table with cargo-features first
        let mut new_table = Table::new();
        new_table.insert("cargo-features", item);
        for (key, val) in doc.as_table().iter() {
            new_table.insert(key, val.clone());
        }
        *doc.as_table_mut() = new_table;

        // Add a blank line after cargo-features only if there isn't one already
        if let Some(Item::Value(value)) = doc.get_mut("cargo-features") {
            let current_suffix = value.decor().suffix();
            // Check if suffix exists and contains newline
            let needs_newline = match current_suffix {
                Some(suffix_str) => !suffix_str.as_str().unwrap_or("").contains('\n'),
                None => true,
            };
            
            if needs_newline {
                value.decor_mut().set_suffix("\n");
            }
        }

        tokio::fs::write(&manifest_path, doc.to_string()).await?;
        println!("Updated {}", manifest_path.to_path_buf());
    }

    Ok(())
}
