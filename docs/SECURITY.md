# Politique de sécurité — Gideonne_1

## Modèle de sécurité

Gideonne est conçu pour fonctionner **entièrement en local**. Aucune donnée n'est envoyée vers des serveurs tiers.

## Données stockées

| Type | Emplacement | Chiffrement |
|------|-------------|-------------|
| Conversations | `~/.gideonne/gideonne.db` (SQLite) | Non (local uniquement) |
| Configuration | `~/.gideonne/config.json` | Non |
| Clés API tierces | `~/.gideonne/secrets.enc` | Oui (AES-256-GCM) |
| Journaux | `~/.gideonne/logs/` | Non |

## Signaler une vulnérabilité

Email : yiyenathanael@gmail.com

## Dépendances de sécurité

| Crate | Usage | Version |
|-------|-------|---------|
| `aes-gcm` | Chiffrement AES-256-GCM | 0.10 |
| `rand` | Génération de nonces sécurisés | 0.8 |
| `rusqlite` | SQLite avec requêtes paramétrées | 0.31 |
