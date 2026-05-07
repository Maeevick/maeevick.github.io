# maeevick.com

![Build and Deploy](https://github.com/Maeevick/maeevick.github.io/workflows/build_and_deploy/badge.svg)
![Grimble Running Build](https://github.com/Maeevick/maeevick.github.io/workflows/Build%20and%20Deploy%20Grimble%20Running%20Mini%20Game/badge.svg)
![License](https://img.shields.io/badge/License-MIT%20%2B%20CC%20BY--NC--SA%204.0-blue.svg)

Personal site of Aurel Estoup (Maeevick) — CTPO-as-a-Service for pre-PMF startups & Tech4Good, crossroads of professional and creative worlds.

https://www.maeevick.com

## Structure

Four sections:

- **The Tavern** (`/`) — Home, professional pitch, latest content
- **Trix's Lab** (`/trix-lab/`) — Technical experiments, side projects, games
- **Grimble's Codex** (`/grimble-codex/`) — World-building, characters, lore
- **Veil's Vault** (`/veil-vault/`) — Cybersecurity research, vulnerability findings, bug bounties

## Getting Started

### Prerequisites

- **Zola** (v0.20.0+)

### Quick Start

```bash
git clone https://github.com/Maeevick/maeevick.github.io.git
cd maeevick.github.io/app

# Serve locally
zola serve
# → http://127.0.0.1:1111

# Build for production
zola build

# Check for issues
zola check
```

## Tech Stack

### Core

- **Zola** — Static site generator (Rust)
- **SCSS** — Styling via CSS custom properties, dark-mode first design system
- **Tera** — Zola's templating engine, macro component library in `templates/macros/`

### Design System

- **Fonts** — VT323 (display/headings), Space Mono (body), both self-hosted as woff2
- **Colors** — Orange primary, magenta + blue accents, pink→orange gradient throughout
- **Per-section accent** — Trix=orange, Grimble=blue, Veil=magenta via `body.section-*` class
- **No JavaScript** for UI — CSS-only hamburger menu, dark mode permanent (system default)

### Performance & Accessibility

- **Images** — WebP with JPEG fallback via `<picture>`, resized to display dimensions
- **Lighthouse** — Targeting 100/100 across all categories on production build
- **i18n** — Bilingual EN/FR with locale-aware date formatting

### Games & Interactive Content

- **Rust + Bevy Engine** — Game development
- **WebAssembly (WASM)** — Browser-compatible game runtime
- **Scaleway Object Storage** — Cloud hosting for WASM builds

## Repository Layout

```
app/                  # Zola site root
  content/            # Markdown content (EN + FR .fr.md pairs)
  templates/          # Tera templates + macros/components.html
  sass/               # SCSS source
    components/       # Per-component partials
  static/             # Fonts, images, favicon
grimble-running/      # Rust/Bevy game source
```

## License

Dual license:

- **Code** (templates, SCSS, config) — [MIT License](LICENSE)
- **Creative content** (texts, stories, lore, images) — [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)

Questions? Open an issue or reach out on [LinkedIn](https://www.linkedin.com/in/aurel-estoup/) or [BlueSky](https://bsky.app/profile/maeevick.bsky.social).
