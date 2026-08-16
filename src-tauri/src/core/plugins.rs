// core/plugins.rs — Système de plugins extensibles
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataPlugin {
    pub id: String,
    pub nom: String,
    pub version: String,
    pub description: String,
    pub auteur: String,
    pub actif: bool,
}

static PLUGINS: std::sync::OnceLock<std::sync::Mutex<HashMap<String, MetadataPlugin>>> = std::sync::OnceLock::new();

/// Charge les plugins depuis ~/.gideonne/plugins/
pub async fn charger_plugins(app: &AppHandle) -> Result<()> {
    let registre = PLUGINS.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
    let mut chemin_plugins = app.path().app_data_dir().map_err(|e| anyhow::anyhow!("Err: {}", e))?;
    chemin_plugins.push("plugins");
    if !chemin_plugins.exists() {
        std::fs::create_dir_all(&chemin_plugins)?;
        return Ok(());
    }
    let mut nb = 0;
    if let Ok(entrees) = std::fs::read_dir(&chemin_plugins) {
        for entree in entrees.flatten() {
            if entree.path().is_dir() {
                let manifest = entree.path().join("manifest.json");
                if manifest.exists() {
                    if let Ok(meta) = charger_manifest(&manifest) {
                        if let Ok(mut reg) = registre.lock() { reg.insert(meta.id.clone(), meta); }
                        nb += 1;
                    }
                }
            }
        }
    }
    tracing::info!("{} plugin(s) chargé(s).", nb);
    Ok(())
}

fn charger_manifest(chemin: &std::path::Path) -> Result<MetadataPlugin> {
    let contenu = std::fs::read_to_string(chemin)?;
    Ok(serde_json::from_str(&contenu)?)
}

pub fn lister_plugins() -> Vec<MetadataPlugin> {
    if let Some(registre) = PLUGINS.get() {
        if let Ok(reg) = registre.lock() { return reg.values().cloned().collect(); }
    }
    vec![]
}
