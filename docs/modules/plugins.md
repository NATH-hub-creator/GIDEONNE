# Module Plugins — Gideonne_1

## Structure d'un plugin

```
~/.gideonne/plugins/
└── mon-plugin/
    ├── manifest.json
    └── README.md
```

## Format du manifest.json

```json
{
  "id": "mon-plugin-id",
  "nom": "Mon Plugin",
  "version": "1.0.0",
  "description": "Ce plugin fait X et Y.",
  "auteur": "Votre Nom",
  "actif": true
}
```

## Roadmap

- v0.2.0 : Interface de gestion des plugins
- v0.5.0 : Plugins Rust natifs
- v1.0.0 : Marketplace communautaire
