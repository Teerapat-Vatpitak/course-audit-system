# Course Audit System

> Privacy-first transcript auditing for PSU Computer Science — powered by Rust + WebAssembly.

Upload your PDF transcript, and instantly see how far you are from graduation. **Everything runs in your browser — no server, no uploads, no data leaves your device.**

---

## Quick Start

### Prerequisites

| Tool                         | Version | Purpose                                    |
| ---------------------------- | ------- | ------------------------------------------ |
| [Rust](https://rustup.rs)    | stable  | Compiler + `wasm32-unknown-unknown` target |
| [Trunk](https://trunkrs.dev) | 0.17+   | WASM build tool, dev server & Tailwind CLI |

```bash
# Install the WASM target (one-time)
rustup target add wasm32-unknown-unknown

# Install Trunk (one-time)
cargo install trunk
```

### Development

```bash
trunk serve --open
# → http://localhost:8080   (hot-reload enabled)
```

### Production Build

```bash
trunk build --release
# → dist/   (optimized WASM with opt-level='z', LTO, single codegen-unit)
```

---

## How It Works

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Upload PDF  │ ──▶ │  PDF.js      │ ──▶ │  Rust Regex  │ ──▶ │  Audit       │
│  (drag/drop) │     │  extracts    │     │  parser      │     │  engine      │
│              │     │  raw text    │     │  normalizes  │     │  matches     │
│              │     │  (JS interop)│     │  course data │     │  curriculum  │
└──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
                                                                      │
                                                               ┌──────▼──────┐
                                                               │  Leptos UI  │
                                                               │  renders    │
                                                               │  results    │
                                                               └─────────────┘
```

1. **Upload** — Drag & drop or click to select your unofficial transcript PDF.
2. **Extract** — PDF.js (running in the browser) pulls raw text from each page.
3. **Parse** — A Rust regex parser normalizes course codes (e.g. `890-001` → `890-101`), extracts names, credits, and grades. Special topics (344-496–499) are greedy-numbered for deduplication.
4. **Audit** — The engine validates courses against the full PSU CS curriculum:
   - **General Education** — 7 strands (with sub-groups, sequential pairs, choose-one rules) + 6 elective sub-categories.
   - **Major** — Basic Science → Core → Capstone → Electives (with cluster completion tracking).
   - **Free Electives** — Any remaining passing courses auto-detected.
5. **Display** — Donut charts, progress bars, expandable course lists, color-coded grades, and missing-requirement breakdowns.

---

## Project Structure

```
course-audit-system/
├── Cargo.toml                 # Rust dependencies & release profile
├── index.html                 # Trunk entry point, PDF.js setup
├── style.css                  # Tailwind directives + custom utilities
├── tailwind.config.js         # Design tokens, animations, shadows
│
├── src/
│   ├── main.rs                # App component, state management, UI layout
│   ├── models.rs              # All data types + shared utility functions
│   │
│   ├── components/
│   │   └── category_card.rs   # Expandable accordion with grade colors
│   │
│   ├── data/
│   │   ├── gen_ed.rs          # GenEd curriculum (7 strands + electives)
│   │   └── major.rs           # Major curriculum (science, core, capstone, electives)
│   │
│   └── logic/
│       ├── parser.rs          # PDF text → Vec<ParsedCourse>
│       └── auditor.rs         # Curriculum matching & credit calculation
│
└── .github/
    └── workflows/
        └── deploy.yml         # Auto-deploy to GitHub Pages on push to main
```

---

## Curriculum Coverage

| Category              | Required    | Details                                                                                            |
| --------------------- | ----------- | -------------------------------------------------------------------------------------------------- |
| **General Education** | 30 cr       | 7 strands (choose-all, choose-one, choose-sequential-pair, sub-groups) + 6 elective sub-categories |
| **Basic Science**     | 12 cr       | Math, Physics, Chemistry foundations                                                               |
| **Core Courses**      | 56 cr       | All required CS courses                                                                            |
| **Capstone**          | 3–6 cr      | Senior Project or Co-operative Education (choose 1)                                                |
| **Major Electives**   | 12 cr       | Domain-based clusters — must complete at least 2 full clusters                                     |
| **Free Electives**    | 6 cr        | Auto-detected from unmatched passing courses                                                       |
| **Total**             | **~132 cr** | Full B.Sc. (Computer Science) degree                                                               |

### Audit Rules

- **Greedy matching** — Repeatable special topics (344-496 to 344-499) accumulate credits across multiple enrollments.
- **Code normalization** — Legacy codes (890-001 → 890-101, etc.) mapped to current curriculum.
- **Credit validation** — Credits taken from curriculum definition, capped by transcript value, to guard against PDF parsing drift.
- **Deduplication** — Special topics keyed by `code::name`; regular courses keyed by code alone.

---

## Tech Stack

| Layer     | Technology                                | Role                                       |
| --------- | ----------------------------------------- | ------------------------------------------ |
| Language  | **Rust** (2021 edition)                   | Type-safe, fast compilation to WASM        |
| Framework | **Leptos 0.6** (CSR)                      | Reactive UI with fine-grained signals      |
| Build     | **Trunk**                                 | WASM bundler, asset pipeline, dev server   |
| Styling   | **Tailwind CSS**                          | Utility-first, custom design tokens        |
| Fonts     | **Inter Variable** + **JetBrains Mono**   | UI + monospace for codes/numbers           |
| PDF       | **PDF.js 3.11**                           | Client-side text extraction via JS interop |
| Interop   | **wasm-bindgen**, **web-sys**, **js-sys** | Rust ↔ JavaScript bridge                   |

### Release Optimizations

```toml
[profile.release]
opt-level = 'z'    # Optimize for binary size
lto = true         # Link-time optimization
codegen-units = 1  # Single codegen unit for maximum optimization
```

---

## Deployment

### GitHub Pages (Automatic)

Push to `main` → GitHub Actions builds with Trunk → deploys to Pages.

The workflow ([`.github/workflows/deploy.yml`](.github/workflows/deploy.yml)):

1. Checks out code
2. Installs Rust + `wasm32-unknown-unknown` target
3. Runs `trunk build --release --public-url /course-audit-system/`
4. Uploads `dist/` as a Pages artifact
5. Deploys to your GitHub Pages URL

### Manual / Other Hosts

```bash
trunk build --release --public-url /your-base-path/
# Upload the contents of dist/ to any static host
# (Netlify, Vercel, Cloudflare Pages, S3, etc.)
```

---

## Privacy

|                |                                           |
| -------------- | ----------------------------------------- |
| **Processing** | 100% in-browser via WebAssembly           |
| **Network**    | Zero API calls — no server exists         |
| **Storage**    | Nothing persisted — refresh = clean slate |
| **Data**       | Your transcript PDF never leaves the tab  |
| **Code**       | Open source, fully auditable              |

---

## License

MIT
