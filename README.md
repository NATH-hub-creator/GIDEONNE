# Gideonne_1

> Assistant IA local, modulaire et multilingue — propulsé par Rust, Tauri v2 et React 18.

[![Licence MIT](https://img.shields.io/badge/licence-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021_edition-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-v2-purple.svg)](https://tauri.app)
[![React](https://img.shields.io/badge/react-18-61DAFB.svg)](https://reactjs.org)

---

## Présentation

Gideonne_1 est un assistant IA personnel crée en Août 2026 par Le Docteur Nathanaël NAGALO CEO de NAG NAT industries.Cet assistant IA  fonctionne **entièrement en local**, sans dépendance
à des services cloud. Il s'appuie sur Ollama pour l'inférence LLM, intègre des capacités de vision
par ordinateur (YOLO/OpenCV), de reconnaissance et synthèse vocale (Whisper STT / Piper TTS).

### Caractéristiques clés

- **100 % local** : aucune donnée envoyée vers des serveurs tiers
- **Multilingue** : français, anglais, espagnol, mooré, gurunsi, latin
- **Modulaire** : architecture en plugins
- **Sécurisé** : chiffrement AES-256-GCM
- **Extensible** : système de plugins Rust

---

## Prérequis

| Outil | Version minimale |
|-------|------------------|
| Rust  | 1.75+            |
| Node.js | 20+            |
| npm   | 10+              |
| Tauri CLI | 2.0+         |
| Ollama | 0.1.30+        |

---

## Installation

```bash
git clone https://github.com/NATH-hub-creator/GIDEONNE.git
cd GIDEONNE
npm install
npm run tauri dev
```

---

## Architecture

```
Gideonne_1/
├── src-tauri/          # Backend Rust (Tauri)
│   └── src/
│       ├── commands/   # Commandes exposées au frontend
│       ├── core/       # Mémoire, plugins, config, journaux
│       └── i18n/       # Internationalisation backend
├── src/                # Frontend React + TypeScript
│   ├── components/
│   ├── hooks/
│   ├── stores/
│   └── i18n/
├── docs/               # Documentation technique
└── tests/              # Tests unitaires et d'intégration
```

## Modules

| Module | Statut |
|--------|--------|
| AI (Ollama) | Fonctionnel |
| Mémoire (SQLite) | Fonctionnel |
| Système | Fonctionnel |
| Filesystem | Fonctionnel |
| Réseau | Fonctionnel |
| Vision | Stub (v0.3.0) |
| Voix | Stub (v0.2.0) |
| Communication | Stub (v0.4.0) |
| Sécurité | Fonctionnel |
| Plugins | Fonctionnel |
| i18n (6 langues) | Fonctionnel |

## Licence

MIT © 2026 Nathanael (NATH-hub-creator)
