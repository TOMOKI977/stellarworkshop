# Paso 3 · MCP: conecta tu agente a Stellar

Conecta tu agente a dos servidores MCP: Stellar Docs (documentación oficial, sin login) y Raven (docs y datos vivos del ecosistema, con login OAuth).

## Sin IA

```bash
# Claude Code
claude mcp add --transport http --scope user stellar-docs https://developers.stellar.org/mcp
claude mcp add --transport http stellar-raven https://raven.stellar.org/mcp
# luego, dentro del agente: /mcp → Authenticate (Raven)

# Codex
codex mcp add stellar-docs --url https://developers.stellar.org/mcp
codex mcp add stellar-raven --url https://raven.stellar.org/mcp
codex mcp login stellar-raven
```

> ¿Tu herramienta no soporta MCP? Pega https://developers.stellar.org/llms.txt en el chat.

## Fuente oficial

- https://developers.stellar.org/mcp
- https://raven.stellar.org
- https://developers.stellar.org/llms.txt
