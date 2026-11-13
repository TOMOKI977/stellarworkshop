// Structural check for the workshop site. Node built-ins only.
// Usage: node scripts/check-site.mjs
import { readFileSync, readdirSync, existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const siteDir = join(root, 'site');
const promptsDir = join(root, 'prompts');

const SECTION_IDS = [
  'cover', 'quien', 'stack', 'gentle-ai', 'cli',
  'skills', 'mcp', 'demo', 'hackathon', 'gracias',
];
const BASE_URL = 'https://tomoki977.github.io/stellarworkshop/';

const html = readFileSync(join(siteDir, 'index.html'), 'utf8');
const failures = [];
let checks = 0;

function check(ok, message) {
  checks += 1;
  if (!ok) failures.push(message);
}

const decode = (text) => text
  .replace(/&lt;/g, '<')
  .replace(/&gt;/g, '>')
  .replace(/&quot;/g, '"')
  .replace(/&#39;/g, "'")
  .replace(/&nbsp;/g, ' ')
  .replace(/&amp;/g, '&');
const normalize = (text) => text.replace(/\s+/g, ' ').trim();

// 1. Every expected slide exists, in order.
const sectionIds = [...html.matchAll(/<section\b[^>]*\bid="([^"]+)"/g)].map((m) => m[1]);
for (const id of SECTION_IDS) {
  check(sectionIds.includes(id), `missing <section id="${id}"> in site/index.html`);
}
check(
  sectionIds.join(',') === SECTION_IDS.join(','),
  `section order is [${sectionIds.join(', ')}], expected [${SECTION_IDS.join(', ')}]`,
);

// 2. Every referenced QR file exists.
const qrRefs = [...new Set([...html.matchAll(/src="(qr\/[^"]+\.svg)"/g)].map((m) => m[1]))];
check(qrRefs.length > 0, 'no qr/*.svg references found in site/index.html');
for (const ref of qrRefs) {
  check(existsSync(join(siteDir, ref)), `referenced QR file is missing: site/${ref}`);
}

// 3. Every deep-link QR targets an existing section.
const deepLinks = [...html.matchAll(/alt="[^"]*#([\w-]+)"/g)].map((m) => m[1]);
for (const id of deepLinks) {
  check(sectionIds.includes(id), `QR deep link #${id} has no matching section`);
  check(qrRefs.includes(`qr/${id}.svg`), `deep link #${id} is not backed by qr/${id}.svg`);
}
const qrDir = join(siteDir, 'qr');
if (existsSync(qrDir)) {
  for (const file of readdirSync(qrDir).filter((f) => f.endsWith('.svg'))) {
    const id = file.replace(/\.svg$/, '');
    if (['site', 'linktree', 'repo'].includes(id)) continue;
    check(sectionIds.includes(id), `site/qr/${file} points to ${BASE_URL}#${id}, which has no section`);
  }
}

// 4. Every prompt in prompts/*.md appears verbatim (whitespace-normalized) on the site.
const pageText = normalize(decode(html.replace(/<[^>]+>/g, '')));
const promptFiles = readdirSync(promptsDir).filter((f) => f.endsWith('.md')).sort();
check(promptFiles.length > 0, 'no prompts/*.md files found');
for (const file of promptFiles) {
  const source = readFileSync(join(promptsDir, file), 'utf8').replace(/\r\n/g, '\n');
  const match = source.match(/```text\n([\s\S]*?)```/);
  check(Boolean(match), `prompts/${file} has no \`\`\`text block`);
  if (match) {
    check(pageText.includes(normalize(match[1])), `prompt from prompts/${file} does not appear in site/index.html`);
  }
}

if (failures.length > 0) {
  console.error(`FAIL: ${failures.length} of ${checks} checks failed`);
  for (const failure of failures) console.error(`  - ${failure}`);
  process.exit(1);
}
console.log(`OK: ${checks} checks`);
