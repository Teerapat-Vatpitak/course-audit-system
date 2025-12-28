# 🎓 Course Audit System (Rust + Leptos)

[![Rust](https://img.shields.io/badge/Rust-stable-brown?logo=rust)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-654ff0?logo=webassembly)](https://webassembly.org/)
[![Leptos](https://img.shields.io/badge/Leptos-frontend-0ea5e9)](https://www.leptos.dev/)
[![Privacy First](https://img.shields.io/badge/Privacy-First-success)](#privacy)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A client-side WebAssembly application for Computer Science students at Prince of Songkla University (PSU). Upload your transcript (PDF), parse it locally, and audit progress against curriculum requirements (GenEd, Major, Electives)—all in your browser.

![Project Screenshot](path/to/screenshot.png)

---

## About

Course Audit System is a privacy-first tool that evaluates academic progress with zero server dependencies. Built in Rust and compiled to WebAssembly via Leptos, it extracts course entries from transcripts using robust Regex, normalizes codes, and applies deterministic logic to check PSU curriculum requirements.

- **Runs entirely in the browser** using WASM.
- **No data leaves your device**; no server calls or storage.
- **Clear, predictable auditing** rules based on static curriculum data.

---

## Key Features

- **PDF Parsing:** Extracts course data directly from transcript PDFs using Regex.
- **Curriculum Audit:**
  - General Education strands and sub-groups (including sequential pair constraints).
  - Major requirements (Basic Science, Core, Capstone).
  - Electives across domains and clusters.
- **Intelligent Logic:**
  - **Greedy matching** for repeatable courses (Special Topics like `344-496`) so multiple completions accumulate credits correctly.
  - **Free Electives detection** for unmatched but passing courses, using the PDF's parsed credits.
- **Responsive UI:**
  - Fast, signal-based Leptos UI.
  - Accordion-style category cards for clear, collapsible summaries.

---

## Tech Stack

- **Language:** Rust 🦀
- **Framework:** Leptos (WebAssembly)
- **Build Tool:** Trunk
- **Styling:** Tailwind CSS
- **PDF Engine:** PDF.js (via JavaScript interop)
- **Interop:** wasm-bindgen, web-sys

---

## Project Structure

The project uses a modular architecture to separate concerns across data, logic, and UI.

```text
src/
├── models.rs          # Core data structures (Course, Category, AuditResult, curriculum types)
├── data/
│   ├── mod.rs         # Module declarations
│   ├── gen_ed.rs      # Static General Education curriculum + get_gen_ed_curriculum()
│   └── major.rs       # Static Major curriculum + get_major_curriculum()
├── logic/
│   ├── mod.rs         # Module declarations
│   ├── parser.rs      # Transcript parsing + JS interop: extractTextFromPDF
│   └── auditor.rs     # Auditing algorithms (GenEd/Major) + greedy matching + free electives
├── components/
│   ├── mod.rs         # Module declarations
│   └── category_card.rs # UI component for collapsible category cards
└── main.rs            # App entry; wires data, logic, and UI together
```

- **`src/models.rs`:** Core data structures shared across the app (e.g., `Course`, `Category`, `AuditResult`, `GenEdCurriculum`, `MajorCurriculum`).
- **`src/data/`:** Static curriculum sources for GenEd and Major requirements.
- **`src/logic/`:** Core business logic:
  - PDF parsing (Regex), code normalization.
  - Audit algorithms for GenEd/Major, including sub-groups, sequences, and elective domains.
  - Greedy matching for repeatable "Special Topics" courses.
- **`src/components/`:** Reusable UI widgets such as `CategoryCard`.

---

## Getting Started

### Prerequisites

```bash
# Install Rust
# Windows: https://win.rustup.rs/
# macOS/Linux:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM build tool)
cargo install trunk
```

### Clone & Run

```bash
git clone https://github.com/yourusername/course-audit-system.git
cd course-audit-system

# Install dependencies (Rust and Trunk must be installed)
# Then start the dev server
trunk serve --open
```

---

## Deployment to GitHub Pages

This project is configured to deploy automatically to GitHub Pages on every push to `main`:

1. **Setup:**
   - Repository must be public
   - Enable GitHub Pages in repository settings → select "GitHub Actions" as source
   - The `.github/workflows/deploy.yml` file handles the build and deploy

2. **View Live:**
   - Visit: `https://yourusername.github.io/course-audit-system/`

3. **Manual Deployment:**
   ```bash
   trunk build --release
   # Then manually push the dist/ folder to gh-pages branch
   ```

---

## Usage

1. **Launch** the app (locally or visit the GitHub Pages link):
   - Local: `trunk serve --open`
   - GitHub Pages: Visit deployed URL

2. **Upload** your transcript PDF:
   - Drag-and-drop onto the upload area, or
   - Click to browse and select the file

3. **Review** results:
   - Total credits earned
   - Collapsible category cards for:
     - **General Education**: Organized by 6 strands
     - **Major Courses**: Organized by Basic Science, Core, Capstone, and Electives
     - **Free Electives**: Unmatched passing courses
   - Missing required courses to plan next steps
   - Progress bars showing completion percentage for each category

---

## Privacy & Security

All processing is performed **locally in your browser** via WebAssembly:
- ✅ **No servers involved** - Your transcript never touches any external server
- ✅ **No data uploads** - Everything stays on your device
- ✅ **No external storage** - No cookies or local storage of sensitive data
- ✅ **Open source** - Code is auditable and transparent

Your transcript PDF data never leaves your device.

---

## Development

### Project Organization

- **`src/main.rs`** - App entry point, state management, and UI layout
- **`src/models.rs`** - Core data structures (Course, Category, AuditResult, etc.)
- **`src/components/`** - Leptos UI components (CategoryCard)
- **`src/data/`** - Static curriculum definitions (GenEd, Major)
- **`src/logic/`** - Business logic (PDF parser, auditor engine)
- **`.github/workflows/`** - GitHub Actions for CI/CD and deployment

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
trunk build --release
# Output in dist/ directory (optimized WASM)
```

---

## Roadmap

- [ ] Dark Mode and improved accessibility
- [ ] Export audit results (JSON/CSV/PDF)
- [ ] Configurable curriculum versions by year
- [ ] Unit tests for parsing and auditing logic
- [ ] i18n support (Thai/English UI)
- [ ] Support for other departments/curricula
- [ ] Course recommendations based on progress

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Areas for Contribution
- Bug fixes and improvements to PDF parsing
- Enhanced audit logic and edge case handling
- UI/UX improvements
- Documentation
- Localization (Thai language support)

---

## License & Credits

- **License:** MIT. See `LICENSE`.
- **Author:** Prince of Songkla University Computer Science Students
- **Built with:**
  - [Leptos](https://www.leptos.dev/) - Rust frontend framework
  - [Trunk](https://trunkrs.dev/) - WASM build tool
  - [Tailwind CSS](https://tailwindcss.com/) - Styling framework
  - [PDF.js](https://mozilla.github.io/pdf.js/) - Client-side PDF text extraction
  - [wasm-bindgen](https://rustwasm.org/docs/wasm-bindgen/) - Rust ↔ JavaScript interop
  - [web-sys](https://rustwasm.org/docs/wasm-bindgen/reference/web-sys/) - Web APIs

---

**Questions or Issues?** Please open an issue on GitHub!
