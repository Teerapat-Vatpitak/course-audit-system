# Course Audit System

Privacy-first WebAssembly app for auditing academic transcripts against PSU Computer Science curriculum. Upload PDF → Parse locally → Audit instantly.

## Quick Start

```bash
# Prerequisites: Rust, Node.js, Trunk
trunk serve --open    # Start dev server at http://localhost:8080
trunk build --release # Production build → dist/
```

## Features

- **PDF Parsing**: Extracts course codes, names, credits, grades using Regex
- **General Education Audit**: 7 strands + 6 electives (30 credits total)
- **Major Audit**: Basic Science (12cr) + Core (56cr) + Capstone + Electives (12cr)
- **Smart Matching**: Greedy matching for repeatable courses, code normalization, validation
- **Free Electives**: Auto-detects unmatched passing courses
- **100% Client-Side**: No servers, no data uploads

## Project Structure

```
src/
├── main.rs              # UI, state, audit orchestration
├── models.rs            # Data types
├── components/
│   └── category_card.rs # Collapsible category cards
├── data/
│   ├── gen_ed.rs        # GenEd curriculum
│   └── major.rs         # Major curriculum
└── logic/
    ├── parser.rs        # PDF → course data
    └── auditor.rs       # Curriculum validation
```

## Curriculum Overview

| Category  | Credits | Details                                                                   |
| --------- | ------- | ------------------------------------------------------------------------- |
| **GenEd** | 30      | 7 strands + sub-groups + sequential pairs + 6 electives                   |
| **Major** | 96      | 12 Basic Science + 56 Core + 3-6 Capstone + 12 Electives (2 clusters min) |

## How It Works

1. Upload PDF transcript → JavaScript PDF.js extracts text
2. Rust regex parser normalizes and structures course data
3. Audit engine matches courses to curriculum requirements
4. UI displays progress, missing courses, and free electives

## Build & Deploy

```bash
# Development
trunk serve --open

# Production
trunk build --release
# Push dist/ to GitHub Pages or static host
```

Deploy automatically: `.github/workflows/deploy.yml` handles GitHub Actions build on push to main.

## Privacy

✅ All processing in browser (WASM)  
✅ No servers, no uploads  
✅ Transcript never leaves your device  
✅ Open source, auditable code

## Tech Stack

- **Rust** + **Leptos** (WASM frontend)
- **Trunk** (build tool)
- **Tailwind CSS** (styling)
- **PDF.js** (text extraction)
- **wasm-bindgen**, **web-sys** (JS interop)

## License

MIT
