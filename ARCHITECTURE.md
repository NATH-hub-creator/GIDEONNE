# Architecture Technique — Gideonne_1

## Vue d'ensemble

```
+-----------------------------------------------------------+
|                        GIDEONNE_1                         |
|                                                           |
|  +-----------------------------------------------------+  |
|  |              FRONTEND (React 18 + TS)               |  |
|  |                                                     |  |
|  |  ChatWindow | Sidebar | StatusBar | Settings | Voice|  |
|  |  Stores (Zustand) --- Hooks --- i18n (6 langues)   |  |
|  +---------------------+---------------------------------+  |
|                        | Tauri IPC (invoke/emit)         |
|  +---------------------v---------------------------------+  |
|  |              BACKEND (Rust + Tauri v2)               |  |
|  |                                                     |  |
|  |  [AI/Ollama] [Systeme] [Fichiers] [Reseau]         |  |
|  |  [Vision*]  [Voix*]   [Comms*]   [Securite]       |  |
|  |  (* = stub, implementation prevue)                 |  |
|  |                                                     |  |
|  |  CORE: Memoire (SQLite) | Plugins | Config | Logs  |  |
|  +-----------------------------------------------------+  |
|                        |                                  |
|  +---------------------v---------------------------------+  |
|  |                STOCKAGE LOCAL                        |  |
|  |  gideonne.db (SQLite) | config.json | plugins/      |  |
|  +-----------------------------------------------------+  |
+-----------------------------------------------------------+
```

## Flux de données — Conversation IA

```
Utilisateur saisit texte
        |
        v
ChatWindow.tsx (React)
        | invoke("envoyer_message")
        v
commands/ai.rs (Rust)
        | HTTP POST /api/chat
        v
Ollama (LLM local)
        | reponse JSON
        v
core/memory.rs -> sauvegarde SQLite
```

## Module Memoire (SQLite)

```sql
CREATE TABLE conversations (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    titre         TEXT    NOT NULL,
    cree_le       TEXT    NOT NULL,
    mis_a_jour_le TEXT    NOT NULL
);

CREATE TABLE messages (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id INTEGER NOT NULL REFERENCES conversations(id),
    role            TEXT    NOT NULL,
    contenu         TEXT    NOT NULL,
    cree_le         TEXT    NOT NULL
);
```

## Technologies

| Couche | Technologie |
|--------|-------------|
| Desktop | Tauri v2 |
| Backend | Rust 2021 |
| Frontend | React 18 + TS |
| Etat | Zustand |
| Build | Vite |
| DB locale | SQLite / rusqlite |
| LLM | Ollama |
| i18n | i18next |
