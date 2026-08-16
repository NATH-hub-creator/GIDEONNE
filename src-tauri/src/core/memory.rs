// core/memory.rs — Persistance SQLite : conversations et messages
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle};
use rusqlite::{Connection, params};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: i64,
    pub titre: String,
    pub cree_le: String,
    pub mis_a_jour_le: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i64,
    pub conversation_id: i64,
    pub role: String,
    pub contenu: String,
    pub cree_le: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsCreationConv { pub titre: String }

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsAjoutMessage {
    pub conversation_id: i64,
    pub role: String,
    pub contenu: String,
}

/// Initialise la base de données SQLite
pub async fn initialiser_db(app: &AppHandle) -> Result<()> {
    let chemin_db = obtenir_chemin_db(app)?;
    let conn = Connection::open(&chemin_db)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS conversations (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            titre         TEXT    NOT NULL,
            cree_le       TEXT    NOT NULL DEFAULT (datetime('now')),
            mis_a_jour_le TEXT    NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS messages (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            conversation_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
            role            TEXT    NOT NULL CHECK(role IN ('user', 'assistant', 'system')),
            contenu         TEXT    NOT NULL,
            cree_le         TEXT    NOT NULL DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_messages_conv ON messages(conversation_id);
    ")?;
    tracing::info!("Base SQLite initialisée : {}", chemin_db.display());
    Ok(())
}

/// Crée une nouvelle conversation
#[command]
pub async fn creer_conversation(app: AppHandle, params: ParamsCreationConv) -> Result<i64, String> {
    let chemin = obtenir_chemin_db(&app).map_err(|e| e.to_string())?;
    let conn = Connection::open(&chemin).map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO conversations (titre) VALUES (?1)", params![params.titre]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

/// Ajoute un message à une conversation
#[command]
pub async fn ajouter_message(app: AppHandle, params: ParamsAjoutMessage) -> Result<i64, String> {
    let chemin = obtenir_chemin_db(&app).map_err(|e| e.to_string())?;
    let conn = Connection::open(&chemin).map_err(|e| e.to_string())?;
    conn.execute("UPDATE conversations SET mis_a_jour_le = datetime('now') WHERE id = ?1", params![params.conversation_id]).map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO messages (conversation_id, role, contenu) VALUES (?1, ?2, ?3)", params![params.conversation_id, params.role, params.contenu]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

/// Liste toutes les conversations
#[command]
pub async fn lister_conversations(app: AppHandle) -> Result<Vec<Conversation>, String> {
    let chemin = obtenir_chemin_db(&app).map_err(|e| e.to_string())?;
    let conn = Connection::open(&chemin).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, titre, cree_le, mis_a_jour_le FROM conversations ORDER BY mis_a_jour_le DESC").map_err(|e| e.to_string())?;
    let convs = stmt.query_map([], |row| Ok(Conversation { id: row.get(0)?, titre: row.get(1)?, cree_le: row.get(2)?, mis_a_jour_le: row.get(3)? })).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();
    Ok(convs)
}

/// Charge les messages d'une conversation
#[command]
pub async fn charger_conversation(app: AppHandle, conversation_id: i64) -> Result<Vec<Message>, String> {
    let chemin = obtenir_chemin_db(&app).map_err(|e| e.to_string())?;
    let conn = Connection::open(&chemin).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, conversation_id, role, contenu, cree_le FROM messages WHERE conversation_id = ?1 ORDER BY id ASC").map_err(|e| e.to_string())?;
    let messages = stmt.query_map(params![conversation_id], |row| Ok(Message { id: row.get(0)?, conversation_id: row.get(1)?, role: row.get(2)?, contenu: row.get(3)?, cree_le: row.get(4)? })).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();
    Ok(messages)
}

fn obtenir_chemin_db(app: &AppHandle) -> Result<std::path::PathBuf> {
    let mut chemin = app.path().app_data_dir().map_err(|e| anyhow::anyhow!("Err: {}", e))?;
    std::fs::create_dir_all(&chemin)?;
    chemin.push("gideonne.db");
    Ok(chemin)
}
