# Paso 2 · Stellar Skills: contexto antes del código

Instala las Stellar Skills oficiales para que tu agente tenga el contexto correcto de Stellar antes de escribir código.

## Sin IA

```bash
# Claude Code (escríbelo dentro del agente)
/plugin marketplace add stellar/stellar-dev-skill
/plugin install stellar-dev@stellar-dev

# Codex, Cursor, VS Code y otros
npx skills add https://github.com/stellar/stellar-dev-skill
```

> Las skills de la comunidad no las revisa la SDF: léelas antes de instalarlas.

## Fuente oficial

- https://skills.stellar.org
- https://github.com/stellar/stellar-dev-skill
