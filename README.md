# LoreKeeper

**A text adventure game where an AI narrator tells your story.**

You explore the dungeon. Rust owns the world state. An LLM writes the prose. No LLM? No problem — template narration kicks in so the game always works.

## The Game: *The Depths of Thornhold*

```
You descend into the ruins of Thornhold, a fortress long abandoned
to darkness. Somewhere below, the Dungeon Heart pulses with ancient
power. Will you claim it, destroy it, or strike a deal with its keeper?
```

- 14 handcrafted locations + 5 procedurally generated dungeon rooms
- 7 NPCs with memory, relationships, and LLM-powered dialogue
- 25+ items, a crafting system, and hidden secret commands
- 5 quests, multiple endings, and a full achievement system
- Combat, status effects, difficulty modes, and a journal/codex
- Replay system with ghost playthrough comparison
- Visual map editor for creating your own adventure modules

## Tech Stack

| Layer | Tech |
|-------|------|
| Desktop app | [Tauri 2](https://tauri.app/) |
| Game engine | Rust |
| Frontend | React 19 + TypeScript (strict) |
| Narration | [Ollama](https://ollama.com/) (local LLM) with template fallback |
| Persistence | SQLite (saves, stats, achievements, themes) |

## Quick Start

```bash
# Prerequisites: Rust, Node.js, and optionally Ollama

npm install
npm run tauri dev

# For LLM narration (optional)
ollama pull llama3.2
```

## Project Stats

```
Rust source     54 files    ~10,700 lines
Frontend        42 files    TypeScript strict, 0 errors
Rust tests      175 passing
Frontend tests  182 passing (Vitest)
Clippy          0 warnings
Bundle          267 KB JS + 18 KB CSS
```

## Architecture

```
┌─────────────────────────────────────────┐
│              Tauri Shell                │
│  ┌──────────────┐  ┌────────────────┐  │
│  │   React UI   │  │  Rust Engine   │  │
│  │  Terminal +  │◄─┤  Parser        │  │
│  │  Sidebar +   │  │  Executor      │  │
│  │  Map Editor  │  │  World State   │  │
│  └──────┬───────┘  │  Combat/Quests │  │
│         │          │  Crafting      │  │
│         │          └───────┬────────┘  │
│         │                  │           │
│         │          ┌───────▼────────┐  │
│         │          │   Narrator     │  │
│         ◄──────────┤  Ollama / LLM  │  │
│      events        │  or Templates  │  │
│                    └────────────────┘  │
└─────────────────────────────────────────┘
```

## License

MIT
