# Guide de contribution — Gideonne_1

Merci de vouloir contribuer à Gideonne ! Ce guide explique comment participer au projet.

## Prérequis

- Rust 1.75+ (`rustup update stable`)
- Node.js 20+ et npm 10+
- Tauri CLI v2 (`npm install -g @tauri-apps/cli`)
- Git configuré avec ton nom et email

## Démarrage rapide

```bash
# 1. Fork et clone
git clone https://github.com/NATH-hub-creator/GIDEONNE.git
cd GIDEONNE

# 2. Installer les dépendances
npm install

# 3. Lancer en mode développement
npm run tauri dev
```

## Conventions de code

### Rust

- **Style** : `cargo fmt` avant chaque commit
- **Linting** : `cargo clippy -- -D warnings` ne doit pas produire d'avertissements
- **Commentaires** : en français, en-tête de module obligatoire
- **Erreurs** : utilise `anyhow::Result` pour les fonctions publiques

### TypeScript / React

- **Style** : ESLint + Prettier configurés
- **Composants** : un fichier par composant, noms en PascalCase
- **Hooks** : préfixe `use`, noms en camelCase
- **Imports** : utilise les alias `@/` (ex: `@/components/ChatWindow`)

## Workflow Git

1. Crée une branche depuis `main` : `git checkout -b feat/ma-fonctionnalite`
2. Format du commit : `type: description courte`
   - `feat:` nouvelle fonctionnalité
   - `fix:` correction de bug
   - `docs:` documentation
   - `test:` ajout ou modification de tests
3. Ouvre une Pull Request vers `main`

## Tests

```bash
cargo test --workspace
npm test
```
