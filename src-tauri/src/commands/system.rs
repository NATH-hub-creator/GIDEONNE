// ============================================================
// commands/system.rs — Commandes système (shell, processus)
// ============================================================

use serde::{Deserialize, Serialize};
use tauri::command;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultatCommande {
    pub sortie: String,
    pub erreur: String,
    pub code_retour: i32,
    pub succes: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoProcessus {
    pub pid: u32,
    pub nom: String,
}

/// Exécute une commande shell autorisée et retourne stdout/stderr
#[command]
pub async fn executer_commande(commande: String, args: Vec<String>) -> ResultatCommande {
    tracing::info!("Exécution commande : {} {:?}", commande, args);
    let commandes_autorisees = ["ls", "pwd", "echo", "cat", "date", "uname", "df", "du"];
    if !commandes_autorisees.contains(&commande.as_str()) {
        return ResultatCommande {
            sortie: String::new(),
            erreur: format!("Commande '{}' non autorisée.", commande),
            code_retour: -1,
            succes: false,
        };
    }
    match Command::new(&commande).args(&args).output() {
        Ok(output) => ResultatCommande {
            sortie: String::from_utf8_lossy(&output.stdout).to_string(),
            erreur: String::from_utf8_lossy(&output.stderr).to_string(),
            code_retour: output.status.code().unwrap_or(-1),
            succes: output.status.success(),
        },
        Err(e) => ResultatCommande {
            sortie: String::new(),
            erreur: e.to_string(),
            code_retour: -1,
            succes: false,
        },
    }
}

/// Liste les processus en cours (Linux seulement)
#[command]
pub async fn lister_processus() -> Vec<InfoProcessus> {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let mut processus = Vec::new();
        if let Ok(entrees) = fs::read_dir("/proc") {
            for entree in entrees.flatten() {
                let nom_fichier = entree.file_name();
                let nom_str = nom_fichier.to_string_lossy();
                if let Ok(pid) = nom_str.parse::<u32>() {
                    let chemin_comm = format!("/proc/{}/comm", pid);
                    let nom = fs::read_to_string(chemin_comm).unwrap_or_default().trim().to_string();
                    if !nom.is_empty() { processus.push(InfoProcessus { pid, nom }); }
                }
            }
        }
        processus.sort_by_key(|p| p.pid);
        processus
    }
    #[cfg(not(target_os = "linux"))]
    { vec![] }
}
