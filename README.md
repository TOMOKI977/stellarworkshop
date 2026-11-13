# Build With Stellar + AI

Presentación y guía del workshop **Build With Stellar + AI** (Cochabamba, 13 de noviembre de 2026), antesala del National Startups Hackathon by Stellar. El sitio **es** la presentación: el expositor la proyecta y tú escaneas el QR para abrir la misma diapositiva en tu celular y copiar comandos y prompts.

**En vivo:** https://tomoki977.github.io/stellarworkshop/

Cada paso tiene dos caminos:

| Camino | Qué haces |
|--------|-----------|
| **Sin IA** | Escribes los comandos tú mismo. |
| **Con IA** | Le das un prompt a tu agente y él ejecuta. |

## Estructura

| Ruta | Contenido |
|------|-----------|
| `site/` | El sitio estático: `index.html`, `styles.css`, `app.js` y `qr/*.svg`. Sin build. |
| `commands/` | Los comandos "Sin IA" de cada paso, con enlaces a la documentación oficial. |
| `prompts/` | Los prompts "Con IA" de cada paso y qué esperar de ellos. |
| `scripts/gen_qr.py` | Genera los códigos QR en `site/qr/`. |
| `scripts/check-site.mjs` | Verificación estructural del sitio. |
| `.github/workflows/pages.yml` | Publica `site/` en GitHub Pages. |

## Presentar

1. Abre https://tomoki977.github.io/stellarworkshop/ (o `site/index.html` en local) en una pantalla de 1024 px o más.
2. Navega con las flechas, `PageUp`/`PageDown`, `Espacio`, `Inicio` y `Fin`.
3. Presiona `f` para pantalla completa.

Cada diapositiva tiene su enlace directo, por ejemplo `#cli` o `#mcp`. En el celular el sitio se recorre con scroll normal.

## Regenerar los QR

Requiere Python con la librería `qrcode` (`pip install qrcode`).

```bash
python scripts/gen_qr.py
```

Si cambias la URL pública, edita `BASE_URL` al inicio del script.

## Verificar

Requiere Node 18 o superior.

```bash
node scripts/check-site.mjs
```

Comprueba que existan todas las secciones y los QR referenciados, que cada QR de diapositiva apunte a una sección real y que cada prompt de `prompts/` aparezca igual en el sitio. Imprime `OK: N checks` si todo está bien.

## Publicar

1. Crea el repositorio público `TOMOKI977/stellarworkshop` en GitHub.
2. Sube la rama `main`.
3. En **Settings → Pages → Source**, elige **GitHub Actions**.

Cada push a `main` vuelve a publicar el sitio.
