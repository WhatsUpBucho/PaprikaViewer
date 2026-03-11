# Paprika Viewer

I wanted to print Paprika recipes from my Mac, so I vibe coded this up with Claude's help.

A read-only macOS desktop app for browsing your [Paprika 3](https://www.paprikaapp.com) recipe library. Built with [Tauri 2](https://tauri.app) (Rust backend) and React/TypeScript.

## Screenshots

### Recipe Library
![Recipe library grid view showing recipe thumbnails with names and star ratings, category sidebar on the left](screenshots/list-view.png)

### Recipe Detail
![Recipe detail modal showing photo, title, print button, ingredients and directions](screenshots/detail-view.png)

## Features

- **Browse all recipes** in a thumbnail grid, sorted alphabetically
- **Search** with real-time filtering and autocomplete suggestions
- **Filter by category** — combinable with name search
- **Recipe detail view** — photo, description, ingredients, directions, notes, nutritional info, and source link
- **Print support** — native macOS print dialog, formatted for 8.5×11" letter paper with 1-inch margins
- **Auto-sync on launch** — hash-based diffing so only changed recipes are downloaded
- **Offline-capable** — recipes and photos cached locally in SQLite
- **Secure login** — Paprika credentials stored in macOS Keychain; auto-login on relaunch

## Requirements

- macOS 12+
- [Rust](https://rustup.rs) (installed via `rustup`)
- Node.js 18+
- A [Paprika 3](https://www.paprikaapp.com) account

## Getting Started

```bash
# Clone the repo
git clone https://github.com/WhatsUpBucho/PaprikaViewer.git
cd PaprikaViewer

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev
```

On first launch, sign in with your Paprika account email and password. Your token is stored in the macOS Keychain — subsequent launches auto-login and sync.

## Tech Stack

| Layer | Technology |
|---|---|
| App shell | Tauri 2 (Rust + WebKit) |
| Backend | Rust — tokio, rusqlite, keyring |
| Paprika API | [rust-paprika-api-fork](https://github.com/tdresser/rust-paprika-api-fork) |
| Database | SQLite (via tokio-rusqlite) |
| Frontend | React 18 + TypeScript + Vite 5 |
| Styling | CSS custom properties, Paprika-inspired theme |

## Building for Distribution

```bash
npm run tauri build
```

The signed `.dmg` will be output to `src-tauri/target/release/bundle/dmg/`.

## License

MIT
