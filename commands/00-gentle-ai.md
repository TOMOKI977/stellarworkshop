# Paso 0 · Prepara tu agente con gentle-ai

Instala gentle-ai para configurar el agente que ya usas (Claude Code, Codex, Cursor, OpenCode, Gemini CLI y más) con memoria persistente (Engram), skills, MCP y un flujo de trabajo ordenado.

## Sin IA

```bash
# macOS
brew install gentleman-programming/tap/gentle-ai
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Gentleman-Programming/gentle-ai/main/scripts/install.sh | bash
# Windows (requiere Go 1.25.10+)
go install github.com/gentleman-programming/gentle-ai/v4/cmd/gentle-ai@v4.1.1

gentle-ai          # elige agentes, componentes y persona
gentle-ai doctor   # verifica la instalación
```

> El asistente `gentle-ai` es interactivo: ejecútalo tú en tu terminal para elegir tu agente.

## Fuente oficial

- https://github.com/Gentleman-Programming/gentle-ai
